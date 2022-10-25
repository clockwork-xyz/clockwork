use {
    crate::config::PluginConfig,
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_client::thread::objects::{Thread, Trigger, TriggerContext},
    clockwork_cron::Schedule,
    dashmap::{DashMap, DashSet},
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, ReplicaAccountInfo, Result as PluginResult,
    },
    solana_program::{clock::Clock, pubkey::Pubkey},
    std::{fmt::Debug, str::FromStr, sync::Arc},
    tokio::runtime::Runtime,
};

pub struct ThreadObserver {
    // Map from slot numbers to the sysvar clock data for that slot.
    pub clocks: DashMap<u64, Clock>,

    // Plugin config values.
    pub config: PluginConfig,

    // The set of the threads that are currently crankable (i.e. have a next_instruction)
    pub crankable_threads: DashSet<Pubkey>,

    // Map from unix timestamps to the list of threads scheduled for that moment.
    pub cron_threads: DashMap<i64, DashSet<Pubkey>>,

    // Map from account pubkeys to the set of threads listening for an account update.
    pub listener_threads: DashMap<Pubkey, DashSet<Pubkey>>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl ThreadObserver {
    pub fn new(config: PluginConfig, runtime: Arc<Runtime>) -> Self {
        Self {
            clocks: DashMap::new(),
            config: config.clone(),
            crankable_threads: DashSet::new(),
            cron_threads: DashMap::new(),
            listener_threads: DashMap::new(),
            runtime,
        }
    }

    pub fn observe_slot(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.clocks
                .retain(|cached_slot, _clock| *cached_slot >= slot);

            // Get the clock for this slot.
            match this.clocks.get(&slot) {
                None => {}
                Some(clock) => {
                    // Index all of the scheduled threads that are now due.
                    // Cache retains all threads that are not yet due.
                    this.cron_threads
                        .retain(|target_timestamp, thread_pubkeys| {
                            let is_due = clock.unix_timestamp >= *target_timestamp;
                            if is_due {
                                for thread_pubkey_ref in thread_pubkeys.iter() {
                                    this.crankable_threads.insert(*thread_pubkey_ref.key());
                                }
                            }
                            !is_due
                        });
                }
            };

            Ok(())
        })
    }

    pub fn observe_clock(self: Arc<Self>, clock: Clock) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.clocks.insert(clock.slot, clock.clone());
            Ok(())
        })
    }

    pub fn observe_account(
        self: Arc<Self>,
        account_pubkey: Pubkey,
        _account_replica: ReplicaAccountInfo,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Move all threads listening to this account into the crankable set.
            this.listener_threads.retain(|pubkey, thread_pubkeys| {
                if account_pubkey.eq(pubkey) {
                    for thread_pubkey in thread_pubkeys.iter() {
                        this.crankable_threads.insert(*thread_pubkey.key());
                    }
                    false
                } else {
                    true
                }
            });

            // TODO This account update could have just been a lamport change (not a data update).
            // TODO To optimize, we need to fetch the thread accounts to verify the data update

            Ok(())
        })
    }

    pub fn observe_thread(
        self: Arc<Self>,
        thread: Thread,
        thread_pubkey: Pubkey,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Remove thread from crankable set
            this.crankable_threads.remove(&thread_pubkey);

            // If the thread is paused, just return without indexing
            if thread.paused {
                return Ok(());
            }

            if thread.next_instruction.is_some() {
                // If the thread has a next instruction, index it as crankable.
                this.crankable_threads.insert(thread_pubkey);
            } else {
                // Otherwise, index the thread according to its trigger type.
                match thread.trigger {
                    Trigger::Account {
                        address,
                        offset: _,
                        size: _,
                    } => {
                        // Index the thread by its trigger's account pubkey.
                        this.listener_threads
                            .entry(address)
                            .and_modify(|v| {
                                v.insert(thread_pubkey);
                            })
                            .or_insert_with(|| {
                                let v = DashSet::new();
                                v.insert(thread_pubkey);
                                v
                            });
                    }
                    Trigger::Cron {
                        schedule,
                        skippable: _,
                    } => {
                        // Find a reference timestamp for calculating the thread's upcoming target time.
                        let reference_timestamp = match thread.exec_context {
                            None => thread.created_at.unix_timestamp,
                            Some(exec_context) => match exec_context.trigger_context {
                                TriggerContext::Cron { started_at } => started_at,
                                _ => {
                                    return Err(GeyserPluginError::Custom(
                                        "Invalid exec context".into(),
                                    ))
                                }
                            },
                        };

                        // Index the thread to its target timestamp
                        match next_moment(reference_timestamp, schedule) {
                            None => {} // The thread does not have any upcoming scheduled target time
                            Some(target_timestamp) => {
                                this.cron_threads
                                    .entry(target_timestamp)
                                    .and_modify(|v| {
                                        v.insert(thread_pubkey);
                                    })
                                    .or_insert_with(|| {
                                        let v = DashSet::new();
                                        v.insert(thread_pubkey);
                                        v
                                    });
                            }
                        }
                    }
                    Trigger::Immediate => {
                        this.crankable_threads.insert(thread_pubkey);
                    }
                }
            }

            Ok(())
        })
    }

    fn spawn<F: std::future::Future<Output = PluginResult<()>> + Send + 'static>(
        self: &Arc<Self>,
        f: impl FnOnce(Arc<Self>) -> F,
    ) -> PluginResult<()> {
        self.runtime.spawn(f(self.clone()));
        Ok(())
    }
}

impl Debug for ThreadObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "thread-observer")
    }
}

fn next_moment(after: i64, schedule: String) -> Option<i64> {
    Schedule::from_str(&schedule)
        .unwrap()
        .next_after(&DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(after, 0),
            Utc,
        ))
        .take()
        .map(|datetime| datetime.timestamp())
}
