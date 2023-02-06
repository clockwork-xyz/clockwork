use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
    sync::Arc,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use clockwork_client::thread::state::{Thread, Trigger, TriggerContext};
use clockwork_cron::Schedule;
use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, Result as PluginResult,
};
use solana_program::{clock::Clock, pubkey::Pubkey};
use tokio::sync::RwLock;

pub struct ThreadObserver {
    // Map from slot numbers to the sysvar clock data for that slot.
    pub clocks: RwLock<HashMap<u64, Clock>>,

    // The set of threads with an account trigger.
    // Map from account pubkeys to the set of threads listening for an account update.
    pub account_threads: RwLock<HashMap<Pubkey, HashSet<Pubkey>>>,

    // The set of threads with a cront trigger.
    // Map from unix timestamps to the list of threads scheduled for that moment.
    pub cron_threads: RwLock<HashMap<i64, HashSet<Pubkey>>>,

    // The set of threads with an immediate trigger.
    pub immediate_threads: RwLock<HashSet<Pubkey>>,

    // The set of accounts that have updated.
    pub updated_accounts: RwLock<HashSet<Pubkey>>,
}

impl ThreadObserver {
    pub fn new() -> Self {
        Self {
            clocks: RwLock::new(HashMap::new()),
            account_threads: RwLock::new(HashMap::new()),
            cron_threads: RwLock::new(HashMap::new()),
            immediate_threads: RwLock::new(HashSet::new()),
            updated_accounts: RwLock::new(HashSet::new()),
        }
    }

    pub async fn process_slot(self: Arc<Self>, slot: u64) -> PluginResult<HashSet<Pubkey>> {
        let mut executable_threads: HashSet<Pubkey> = HashSet::new();

        // Drop old clocks.
        let mut w_clocks = self.clocks.write().await;
        w_clocks.retain(|cached_slot, _clock| *cached_slot >= slot);
        drop(w_clocks);

        // Get the set of threads that were triggered by the current clock.
        let r_clocks = self.clocks.read().await;
        if let Some(clock) = r_clocks.get(&slot) {
            let mut w_cron_threads = self.cron_threads.write().await;
            w_cron_threads.retain(|target_timestamp, thread_pubkeys| {
                let is_due = clock.unix_timestamp >= *target_timestamp;
                if is_due {
                    for pubkey in thread_pubkeys.iter() {
                        executable_threads.insert(*pubkey);
                    }
                }
                !is_due
            });
            drop(w_cron_threads);
        }

        // Get the set of threads were triggered by an account update.
        let mut w_account_threads = self.account_threads.write().await;
        let mut w_updated_accounts = self.updated_accounts.write().await;
        w_updated_accounts.iter().for_each(|account_pubkey| {
            if let Some(thread_pubkeys) = w_account_threads.get(&account_pubkey) {
                thread_pubkeys.iter().for_each(|pubkey| {
                    executable_threads.insert(*pubkey);
                });
                w_account_threads.remove(&account_pubkey);
            }
        });
        w_updated_accounts.clear();
        drop(w_account_threads);
        drop(w_updated_accounts);

        // Get the set of immediate threads.
        let mut w_immediate_threads = self.immediate_threads.write().await;
        w_immediate_threads.iter().for_each(|pubkey| {
            executable_threads.insert(*pubkey);
        });
        w_immediate_threads.clear();
        drop(w_immediate_threads);

        Ok(executable_threads)
    }

    pub async fn observe_clock(self: Arc<Self>, clock: Clock) -> PluginResult<()> {
        let mut w_clocks = self.clocks.write().await;
        w_clocks.insert(clock.slot, clock.clone());
        drop(w_clocks);
        Ok(())
    }

    /// Move all threads listening to this account into the executable set.
    pub async fn observe_account(
        self: Arc<Self>,
        account_pubkey: Pubkey,
        _slot: u64,
    ) -> PluginResult<()> {
        let r_account_threads = self.account_threads.read().await;
        if r_account_threads.contains_key(&account_pubkey) {
            let mut w_updated_accounts = self.updated_accounts.write().await;
            w_updated_accounts.insert(account_pubkey);
            drop(w_updated_accounts);
        }
        drop(r_account_threads);
        Ok(())
    }

    pub async fn observe_thread(
        self: Arc<Self>,
        thread: Thread,
        thread_pubkey: Pubkey,
        slot: u64,
    ) -> PluginResult<()> {
        // If the thread is paused, just return without indexing
        if thread.paused {
            return Ok(());
        }

        info!("indexing thread: {:?} slot: {}", thread_pubkey, slot);
        if thread.next_instruction.is_some() {
            // If the thread has a next instruction, index it as executable.
            let mut w_immediate_threads = self.immediate_threads.write().await;
            w_immediate_threads.insert(thread_pubkey);
            drop(w_immediate_threads);
        } else {
            // Otherwise, index the thread according to its trigger type.
            match thread.trigger {
                Trigger::Account {
                    address,
                    offset: _,
                    size: _,
                } => {
                    // Index the thread by its trigger's account pubkey.
                    let mut w_account_threads = self.account_threads.write().await;
                    w_account_threads
                        .entry(address)
                        .and_modify(|v| {
                            v.insert(thread_pubkey);
                        })
                        .or_insert_with(|| {
                            let mut v = HashSet::new();
                            v.insert(thread_pubkey);
                            v
                        });
                    drop(w_account_threads);
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
                            let mut w_cron_threads = self.cron_threads.write().await;
                            w_cron_threads
                                .entry(target_timestamp)
                                .and_modify(|v| {
                                    v.insert(thread_pubkey);
                                })
                                .or_insert_with(|| {
                                    let mut v = HashSet::new();
                                    v.insert(thread_pubkey);
                                    v
                                });
                            drop(w_cron_threads);
                        }
                    }
                }
                Trigger::Immediate => {
                    let mut w_immediate_threads = self.immediate_threads.write().await;
                    w_immediate_threads.insert(thread_pubkey);
                    drop(w_immediate_threads);
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
