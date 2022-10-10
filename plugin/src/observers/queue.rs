use {
    crate::config::PluginConfig,
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_client::queue::objects::{Queue, Trigger, TriggerContext},
    clockwork_cron::Schedule,
    dashmap::{DashMap, DashSet},
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, ReplicaAccountInfo, Result as PluginResult,
    },
    solana_program::{clock::Clock, pubkey::Pubkey},
    std::{fmt::Debug, str::FromStr, sync::Arc},
    tokio::runtime::Runtime,
};

pub struct QueueObserver {
    // Map from slot numbers to the sysvar clock data for that slot.
    pub clocks: DashMap<u64, Clock>,

    // Plugin config values.
    pub config: PluginConfig,

    // The set of the queues that are currently crankable (i.e. have a next_instruction)
    pub crankable_queues: DashSet<Pubkey>,

    // Map from unix timestamps to the list of queues scheduled for that moment.
    pub cron_queues: DashMap<i64, DashSet<Pubkey>>,

    // Map from account pubkeys to the set of queues listening for an account update.
    pub listener_queues: DashMap<Pubkey, DashSet<Pubkey>>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl QueueObserver {
    pub fn new(config: PluginConfig, runtime: Arc<Runtime>) -> Self {
        Self {
            clocks: DashMap::new(),
            config: config.clone(),
            crankable_queues: DashSet::new(),
            cron_queues: DashMap::new(),
            listener_queues: DashMap::new(),
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
                    // Index all of the scheduled queues that are now due.
                    // Cache retains all queues that are not yet due.
                    this.cron_queues.retain(|target_timestamp, queue_pubkeys| {
                        let is_due = clock.unix_timestamp >= *target_timestamp;
                        if is_due {
                            for queue_pubkey_ref in queue_pubkeys.iter() {
                                this.crankable_queues.insert(*queue_pubkey_ref.key());
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
            // Move all queues listening to this account into the crankable set.
            this.listener_queues.retain(|pubkey, queue_pubkeys| {
                if account_pubkey.eq(pubkey) {
                    for queue_pubkey in queue_pubkeys.iter() {
                        this.crankable_queues.insert(*queue_pubkey.key());
                    }
                    false
                } else {
                    true
                }
            });

            // TODO This account update could have just been a lamport change (not a data update).
            // TODO To optimize, we need to fetch the queue accounts to verify the data update

            Ok(())
        })
    }

    pub fn observe_queue(self: Arc<Self>, queue: Queue, queue_pubkey: Pubkey) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Remove queue from crankable set
            this.crankable_queues.remove(&queue_pubkey);

            // If the queue is paused, just return without indexing
            if queue.paused {
                return Ok(());
            }

            if queue.next_instruction.is_some() {
                // If the queue has a next instruction, index it as crankable.
                this.crankable_queues.insert(queue_pubkey);
            } else {
                // Otherwise, index the queue according to its trigger type.
                match queue.trigger {
                    Trigger::Account {
                        pubkey: account_pubkey,
                    } => {
                        // Index the queue by its trigger's account pubkey.
                        this.listener_queues
                            .entry(account_pubkey)
                            .and_modify(|v| {
                                v.insert(queue_pubkey);
                            })
                            .or_insert_with(|| {
                                let v = DashSet::new();
                                v.insert(queue_pubkey);
                                v
                            });
                    }
                    Trigger::Cron {
                        schedule,
                        skippable: _,
                    } => {
                        // Find a reference timestamp for calculating the queue's upcoming target time.
                        let reference_timestamp = match queue.exec_context {
                            None => queue.created_at.unix_timestamp,
                            Some(exec_context) => match exec_context.trigger_context {
                                TriggerContext::Cron { started_at } => started_at,
                                _ => {
                                    return Err(GeyserPluginError::Custom(
                                        "Invalid exec context".into(),
                                    ))
                                }
                            },
                        };

                        // Index the queue to its target timestamp
                        match next_moment(reference_timestamp, schedule) {
                            None => {} // The queue does not have any upcoming scheduled target time
                            Some(target_timestamp) => {
                                this.cron_queues
                                    .entry(target_timestamp)
                                    .and_modify(|v| {
                                        v.insert(queue_pubkey);
                                    })
                                    .or_insert_with(|| {
                                        let v = DashSet::new();
                                        v.insert(queue_pubkey);
                                        v
                                    });
                            }
                        }
                    }
                    Trigger::Immediate => {
                        this.crankable_queues.insert(queue_pubkey);
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

impl Debug for QueueObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "queue-observer")
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
