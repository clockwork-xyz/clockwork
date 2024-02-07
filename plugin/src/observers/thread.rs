use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
    sync::{atomic::AtomicU64, Arc},
};

use chrono::{DateTime, NaiveDateTime, Utc};
use clockwork_cron::Schedule;
use clockwork_thread_program::state::{Equality, Trigger, TriggerContext, VersionedThread};
use log::info;
use pyth_sdk_solana::PriceFeed;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, Result as PluginResult,
};
use solana_program::{clock::Clock, pubkey::Pubkey};
use tokio::sync::RwLock;

pub struct ThreadObserver {
    // Map from slot numbers to the sysvar clock data for that slot.
    pub clocks: RwLock<HashMap<u64, Clock>>,

    // Integer tracking the current epoch.
    pub current_epoch: AtomicU64,

    // The set of threads with an account trigger.
    // Map from account pubkeys to the set of threads listening for an account update.
    pub account_threads: RwLock<HashMap<Pubkey, HashSet<Pubkey>>>,

    // The set of threads with a cron trigger.
    // Map from unix timestamps to the list of threads scheduled for that moment.
    pub cron_threads: RwLock<HashMap<i64, HashSet<Pubkey>>>,

    // The set of threads with a now trigger.
    pub now_threads: RwLock<HashSet<Pubkey>>,

    // The set of threads with a slot trigger.
    pub slot_threads: RwLock<HashMap<u64, HashSet<Pubkey>>>,

    // The set of threads with an epoch trigger.
    pub epoch_threads: RwLock<HashMap<u64, HashSet<Pubkey>>>,

    // The set of threads with a pyth trigger.
    pub pyth_threads: RwLock<HashMap<Pubkey, HashSet<PythThread>>>,

    // The set of accounts that have updated.
    pub updated_accounts: RwLock<HashSet<Pubkey>>,
}

#[derive(Eq, Hash, PartialEq)]
pub struct PythThread {
    pub thread_pubkey: Pubkey,
    pub equality: Equality,
    pub limit: i64,
}

impl ThreadObserver {
    pub fn new() -> Self {
        Self {
            clocks: RwLock::new(HashMap::new()),
            current_epoch: AtomicU64::new(0),
            account_threads: RwLock::new(HashMap::new()),
            cron_threads: RwLock::new(HashMap::new()),
            now_threads: RwLock::new(HashSet::new()),
            slot_threads: RwLock::new(HashMap::new()),
            epoch_threads: RwLock::new(HashMap::new()),
            pyth_threads: RwLock::new(HashMap::new()),
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
                    self.current_epoch
                        .fetch_max(clock.epoch, std::sync::atomic::Ordering::Relaxed);
                    for pubkey in thread_pubkeys.iter() {
                        executable_threads.insert(*pubkey);
                    }
                }
                !is_due
            });
            drop(w_cron_threads);
        }

        // Get the set of threads were triggered by an account update.
        let r_account_threads = self.account_threads.read().await;
        let mut w_updated_accounts = self.updated_accounts.write().await;
        w_updated_accounts.iter().for_each(|account_pubkey| {
            if let Some(thread_pubkeys) = r_account_threads.get(&account_pubkey) {
                thread_pubkeys.iter().for_each(|pubkey| {
                    executable_threads.insert(*pubkey);
                });
            }
        });
        w_updated_accounts.clear();
        drop(r_account_threads);
        drop(w_updated_accounts);

        // Get the set of threads that were triggered by a slot update.
        let mut w_slot_threads = self.slot_threads.write().await;
        w_slot_threads.retain(|target_slot, thread_pubkeys| {
            let is_due = slot >= *target_slot;
            if is_due {
                for pubkey in thread_pubkeys.iter() {
                    executable_threads.insert(*pubkey);
                }
            }
            !is_due
        });
        drop(w_slot_threads);

        // Get the set of threads that were trigger by an epoch update.
        let mut w_epoch_threads = self.epoch_threads.write().await;
        let current_epoch = self
            .current_epoch
            .load(std::sync::atomic::Ordering::Relaxed);
        w_epoch_threads.retain(|target_epoch, thread_pubkeys| {
            let is_due = current_epoch >= *target_epoch;
            if is_due {
                for pubkey in thread_pubkeys.iter() {
                    executable_threads.insert(*pubkey);
                }
            }
            !is_due
        });
        drop(w_epoch_threads);

        // Get the set of immediate threads.
        let mut w_now_threads = self.now_threads.write().await;
        w_now_threads.iter().for_each(|pubkey| {
            executable_threads.insert(*pubkey);
        });
        w_now_threads.clear();
        drop(w_now_threads);

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

    pub async fn observe_price_feed(
        self: Arc<Self>,
        account_pubkey: Pubkey,
        price_feed: PriceFeed,
    ) -> PluginResult<()> {
        let r_pyth_threads = self.pyth_threads.read().await;
        if let Some(pyth_threads) = r_pyth_threads.get(&account_pubkey) {
            for pyth_thread in pyth_threads {
                match pyth_thread.equality {
                    Equality::GreaterThanOrEqual => {
                        if price_feed
                            .get_price_unchecked()
                            .price
                            .ge(&pyth_thread.limit)
                        {
                            let mut w_now_threads = self.now_threads.write().await;
                            w_now_threads.insert(pyth_thread.thread_pubkey);
                            drop(w_now_threads);
                        }
                    }
                    Equality::LessThanOrEqual => {
                        if price_feed
                            .get_price_unchecked()
                            .price
                            .le(&pyth_thread.limit)
                        {
                            let mut w_now_threads = self.now_threads.write().await;
                            w_now_threads.insert(pyth_thread.thread_pubkey);
                            drop(w_now_threads);
                        }
                    }
                }
            }
        }
        drop(r_pyth_threads);
        Ok(())
    }

    pub async fn observe_thread(
        self: Arc<Self>,
        thread: VersionedThread,
        thread_pubkey: Pubkey,
        slot: u64,
    ) -> PluginResult<()> {
        // If the thread is paused, just return without indexing
        if thread.paused() {
            return Ok(());
        }

        info!("Indexing thread: {:?} slot: {}", thread_pubkey, slot);
        if thread.next_instruction().is_some() {
            // If the thread has a next instruction, index it as executable.
            let mut w_now_threads = self.now_threads.write().await;
            w_now_threads.insert(thread_pubkey);
            drop(w_now_threads);
        } else {
            // Otherwise, index the thread according to its trigger type.
            match thread.trigger() {
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

                    // Threads with account triggers might be immediately executable,
                    // Thus, we should attempt to execute these threads right away without for an account update.
                    let mut w_now_threads = self.now_threads.write().await;
                    w_now_threads.insert(thread_pubkey);
                    drop(w_now_threads);
                }
                Trigger::Cron {
                    schedule,
                    skippable: _,
                } => {
                    // Find a reference timestamp for calculating the thread's upcoming target time.
                    let reference_timestamp = match thread.exec_context() {
                        None => thread.created_at().unix_timestamp,
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
                Trigger::Timestamp { unix_ts } => {
                    let mut w_cron_threads = self.cron_threads.write().await;
                    w_cron_threads
                        .entry(unix_ts)
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
                Trigger::Now => {
                    let mut w_now_threads = self.now_threads.write().await;
                    w_now_threads.insert(thread_pubkey);
                    drop(w_now_threads);
                }
                Trigger::Slot { slot } => {
                    let mut w_slot_threads = self.slot_threads.write().await;
                    w_slot_threads
                        .entry(slot)
                        .and_modify(|v| {
                            v.insert(thread_pubkey);
                        })
                        .or_insert_with(|| {
                            let mut v = HashSet::new();
                            v.insert(thread_pubkey);
                            v
                        });
                    drop(w_slot_threads);
                }
                Trigger::Epoch { epoch } => {
                    let mut w_epoch_threads = self.epoch_threads.write().await;
                    w_epoch_threads
                        .entry(epoch)
                        .and_modify(|v| {
                            v.insert(thread_pubkey);
                        })
                        .or_insert_with(|| {
                            let mut v = HashSet::new();
                            v.insert(thread_pubkey);
                            v
                        });
                    drop(w_epoch_threads);
                }
                Trigger::Pyth {
                    price_feed,
                    equality,
                    limit,
                } => {
                    let mut w_pyth_threads = self.pyth_threads.write().await;
                    w_pyth_threads
                        .entry(price_feed)
                        .and_modify(|v| {
                            v.insert(PythThread {
                                thread_pubkey,
                                equality: equality.clone(),
                                limit,
                            });
                        })
                        .or_insert_with(|| {
                            let mut v = HashSet::new();
                            v.insert(PythThread {
                                thread_pubkey,
                                equality,
                                limit,
                            });
                            v
                        });
                    drop(w_pyth_threads);
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
            .next_after(&DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::from_timestamp_opt(after, 0).unwrap(),
                Utc,
            ))
            .take()
            .map(|datetime| datetime.timestamp()),
    }
}
