use std::{fmt::Debug, str::FromStr, sync::Arc};

use chrono::{DateTime, NaiveDateTime, Utc};
use clockwork_client::thread::state::{Thread, Trigger, TriggerContext};
use clockwork_cron::Schedule;
use dashmap::{DashMap, DashSet};
use rayon::prelude::*;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, Result as PluginResult,
};
use solana_program::{clock::Clock, pubkey::Pubkey};

pub struct ThreadObserver {
    // Map from slot numbers to the sysvar clock data for that slot.
    pub clocks: DashMap<u64, Clock>,

    // The set of threads with a cront trigger.
    // Map from unix timestamps to the list of threads scheduled for that moment.
    pub cron_threads: DashMap<i64, DashSet<Pubkey>>,

    // The set of threads with an account trigger.
    // Map from account pubkeys to the set of threads listening for an account update.
    pub listener_threads: DashMap<Pubkey, DashSet<Pubkey>>,

    // The set of threads with an immediate trigger.
    pub immediate_threads: DashSet<Pubkey>,

    // The set of accounts that have updated.
    pub updated_accounts: DashSet<Pubkey>,
}

impl ThreadObserver {
    pub fn new() -> Self {
        Self {
            clocks: DashMap::new(),
            cron_threads: DashMap::new(),
            immediate_threads: DashSet::new(),
            listener_threads: DashMap::new(),
            updated_accounts: DashSet::new(),
        }
    }

    pub fn process_slot(self: Arc<Self>, slot: u64) -> PluginResult<DashSet<Pubkey>> {
        let executable_threads: DashSet<Pubkey> = DashSet::new();

        // Drop old clocks.
        self.clocks
            .retain(|cached_slot, _clock| *cached_slot >= slot);

        // Get the set of threads that were triggered by the current clock.
        if let Some(clock) = self.clocks.get(&slot) {
            self.cron_threads
                .retain(|target_timestamp, thread_pubkeys| {
                    let is_due = clock.unix_timestamp >= *target_timestamp;
                    if is_due {
                        for thread_pubkey_ref in thread_pubkeys.iter() {
                            executable_threads.insert(*thread_pubkey_ref.key());
                        }
                    }
                    !is_due
                });
        }

        // Get the set of threads were triggered by an account update.
        self.updated_accounts.par_iter().for_each(|account_pubkey| {
            if let Some(thread_pubkeys) = self.listener_threads.get(&account_pubkey) {
                thread_pubkeys.par_iter().for_each(|pubkey| {
                    executable_threads.insert(*pubkey);
                });
                self.listener_threads.remove(&account_pubkey);
            }
        });
        self.updated_accounts.clear();

        // Get the set of immediate threads.
        self.immediate_threads.par_iter().for_each(|pubkey| {
            executable_threads.insert(*pubkey);
        });
        self.immediate_threads.clear();

        Ok(executable_threads)
    }

    pub fn observe_clock(self: Arc<Self>, clock: Clock) -> PluginResult<()> {
        self.clocks.insert(clock.slot, clock.clone());
        Ok(())
    }

    /// Move all threads listening to this account into the executable set.
    pub fn observe_account(
        self: Arc<Self>,
        account_pubkey: Pubkey,
        _slot: u64,
    ) -> PluginResult<()> {
        if self.listener_threads.contains_key(&account_pubkey) {
            self.updated_accounts.insert(account_pubkey);
        }
        Ok(())
    }

    pub fn observe_thread(
        self: Arc<Self>,
        thread: Thread,
        thread_pubkey: Pubkey,
        _slot: u64,
    ) -> PluginResult<()> {
        // If the thread is paused, just return without indexing
        if thread.paused {
            return Ok(());
        }

        if thread.next_instruction.is_some() {
            // If the thread has a next instruction, index it as executable.
            self.immediate_threads.insert(thread_pubkey);
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
                    self.immediate_threads.insert(thread_pubkey);
                }
            }
        }

        Ok(())
    }
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
