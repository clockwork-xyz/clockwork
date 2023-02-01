use std::{fmt::Debug, str::FromStr, sync::Arc};

use chrono::{DateTime, NaiveDateTime, Utc};
use clockwork_client::thread::state::{Thread, Trigger, TriggerContext};
use clockwork_cron::Schedule;
use dashmap::{DashMap, DashSet};
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, ReplicaAccountInfo, Result as PluginResult,
};
use solana_program::{clock::Clock, pubkey::Pubkey};
use tokio::runtime::Runtime;

use crate::config::PluginConfig;

pub struct ThreadObserver {
    // Map from slot numbers to the sysvar clock data for that slot.
    pub clocks: DashMap<u64, Clock>,

    // Plugin config values.
    pub config: PluginConfig,

    // Map from pubkeys of executable threads (i.e. have a next_instruction) to the slots when they became executable.
    pub executable_threads: DashMap<Pubkey, u64>,

    // Map from unix timestamps to the list of threads scheduled for that moment.
    pub cron_threads: DashMap<i64, DashSet<Pubkey>>,

    // Map from account pubkeys to the set of threads listening for an account update.
    pub listener_threads: DashMap<Pubkey, DashSet<Pubkey>>,
    // Tokio runtime for processing async tasks.
    // pub runtime: Arc<Runtime>,
}

impl ThreadObserver {
    pub fn new(config: PluginConfig) -> Self {
        Self {
            clocks: DashMap::new(),
            config: config.clone(),
            executable_threads: DashMap::new(),
            cron_threads: DashMap::new(),
            listener_threads: DashMap::new(),
            // runtime,
        }
    }

    pub fn observe_processed_slot(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        // self.spawn(|this| async move {
        self.clocks
            .retain(|cached_slot, _clock| *cached_slot >= slot);

        // Get the clock for this slot.
        match self.clocks.get(&slot) {
            None => {}
            Some(clock) => {
                // Index all of the scheduled threads that are now due.
                // Cache retains all threads that are not yet due.
                self.cron_threads
                    .retain(|target_timestamp, thread_pubkeys| {
                        let is_due = clock.unix_timestamp >= *target_timestamp;
                        if is_due {
                            for thread_pubkey_ref in thread_pubkeys.iter() {
                                self.executable_threads
                                    .insert(*thread_pubkey_ref.key(), slot);
                            }
                        }
                        !is_due
                    });
            }
        };

        Ok(())
        // })
    }

    pub fn observe_clock(self: Arc<Self>, clock: Clock) -> PluginResult<()> {
        self.clocks.insert(clock.slot, clock.clone());
        Ok(())
    }

    /// Move all threads listening to this account into the executable set.
    pub fn observe_account(self: Arc<Self>, account_pubkey: Pubkey, slot: u64) -> PluginResult<()> {
        if let Some(entry) = self.listener_threads.get(&account_pubkey) {
            for thread_pubkey in entry.value().iter() {
                self.executable_threads.insert(*thread_pubkey, slot);
            }
        }
        self.listener_threads.remove(&account_pubkey);
        Ok(())
    }

    pub fn observe_thread(
        self: Arc<Self>,
        thread: Thread,
        thread_pubkey: Pubkey,
        slot: u64,
    ) -> PluginResult<()> {
        // self.spawn(|this| async move {
        // Remove thread from executable set
        self.executable_threads.remove(&thread_pubkey);

        // If the thread is paused, just return without indexing
        if thread.paused {
            return Ok(());
        }

        if thread.next_instruction.is_some() {
            // If the thread has a next instruction, index it as executable.
            self.executable_threads.insert(thread_pubkey, slot);
        } else {
            // Otherwise, index the thread according to its trigger type.
            match thread.trigger {
                Trigger::Account {
                    address,
                    offset: _,
                    size: _,
                } => {
                    // Index the thread by its trigger's account pubkey.
                    self.listener_threads
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
                            self.cron_threads
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
                    self.executable_threads.insert(thread_pubkey, slot);
                }
            }
        }

        Ok(())
        // })
    }

    // fn spawn<F: std::future::Future<Output = PluginResult<()>> + Send + 'static>(
    //     self: &Arc<Self>,
    //     f: impl FnOnce(Arc<Self>) -> F,
    // ) -> PluginResult<()> {
    //     self.runtime.spawn(f(self.clone()));
    //     Ok(())
    // }
}

impl Debug for ThreadObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "thread-observer")
    }
}

fn next_moment(after: i64, schedule: String) -> Option<i64> {
    match Schedule::from_str(&schedule) {
        Err(_) => None,
        Ok(schedule) => schedule
            .next_after(&DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(after, 0),
                Utc,
            ))
            .take()
            .map(|datetime| datetime.timestamp()),
    }
}
