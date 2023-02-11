use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
    sync::Arc,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use clockwork_client::automation::state::{Trigger, TriggerContext};
use clockwork_cron::Schedule;
use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, Result as PluginResult,
};
use solana_program::{clock::Clock, pubkey::Pubkey};
use tokio::sync::RwLock;

use crate::versioned_automation::VersionedAutomation;

pub struct AutomationObserver {
    // Map from slot numbers to the sysvar clock data for that slot.
    pub clocks: RwLock<HashMap<u64, Clock>>,

    // The set of automations with an account trigger.
    // Map from account pubkeys to the set of automations listening for an account update.
    pub account_automations: RwLock<HashMap<Pubkey, HashSet<Pubkey>>>,

    // The set of automations with a cront trigger.
    // Map from unix timestamps to the list of automations scheduled for that moment.
    pub cron_automations: RwLock<HashMap<i64, HashSet<Pubkey>>>,

    // The set of automations with an immediate trigger.
    pub immediate_automations: RwLock<HashSet<Pubkey>>,

    // The set of accounts that have updated.
    pub updated_accounts: RwLock<HashSet<Pubkey>>,
}

impl AutomationObserver {
    pub fn new() -> Self {
        Self {
            clocks: RwLock::new(HashMap::new()),
            account_automations: RwLock::new(HashMap::new()),
            cron_automations: RwLock::new(HashMap::new()),
            immediate_automations: RwLock::new(HashSet::new()),
            updated_accounts: RwLock::new(HashSet::new()),
        }
    }

    pub async fn process_slot(self: Arc<Self>, slot: u64) -> PluginResult<HashSet<Pubkey>> {
        let mut executable_automations: HashSet<Pubkey> = HashSet::new();

        // Drop old clocks.
        let mut w_clocks = self.clocks.write().await;
        w_clocks.retain(|cached_slot, _clock| *cached_slot >= slot);
        drop(w_clocks);

        // Get the set of automations that were triggered by the current clock.
        let r_clocks = self.clocks.read().await;
        if let Some(clock) = r_clocks.get(&slot) {
            let mut w_cron_automations = self.cron_automations.write().await;
            w_cron_automations.retain(|target_timestamp, automation_pubkeys| {
                let is_due = clock.unix_timestamp >= *target_timestamp;
                if is_due {
                    for pubkey in automation_pubkeys.iter() {
                        executable_automations.insert(*pubkey);
                    }
                }
                !is_due
            });
            drop(w_cron_automations);
        }

        // Get the set of automations were triggered by an account update.
        let mut w_account_automations = self.account_automations.write().await;
        let mut w_updated_accounts = self.updated_accounts.write().await;
        w_updated_accounts.iter().for_each(|account_pubkey| {
            if let Some(automation_pubkeys) = w_account_automations.get(&account_pubkey) {
                automation_pubkeys.iter().for_each(|pubkey| {
                    executable_automations.insert(*pubkey);
                });
                w_account_automations.remove(&account_pubkey);
            }
        });
        w_updated_accounts.clear();
        drop(w_account_automations);
        drop(w_updated_accounts);

        // Get the set of immediate automations.
        let mut w_immediate_automations = self.immediate_automations.write().await;
        w_immediate_automations.iter().for_each(|pubkey| {
            executable_automations.insert(*pubkey);
        });
        w_immediate_automations.clear();
        drop(w_immediate_automations);

        Ok(executable_automations)
    }

    pub async fn observe_clock(self: Arc<Self>, clock: Clock) -> PluginResult<()> {
        let mut w_clocks = self.clocks.write().await;
        w_clocks.insert(clock.slot, clock.clone());
        drop(w_clocks);
        Ok(())
    }

    /// Move all automations listening to this account into the executable set.
    pub async fn observe_account(
        self: Arc<Self>,
        account_pubkey: Pubkey,
        _slot: u64,
    ) -> PluginResult<()> {
        let r_account_automations = self.account_automations.read().await;
        if r_account_automations.contains_key(&account_pubkey) {
            let mut w_updated_accounts = self.updated_accounts.write().await;
            w_updated_accounts.insert(account_pubkey);
            drop(w_updated_accounts);
        }
        drop(r_account_automations);
        Ok(())
    }

    pub async fn observe_automation(
        self: Arc<Self>,
        automation: VersionedAutomation,
        automation_pubkey: Pubkey,
        slot: u64,
    ) -> PluginResult<()> {
        // If the automation is paused, just return without indexing
        if automation.paused() {
            return Ok(());
        }

        info!(
            "indexing automation: {:?} slot: {}",
            automation_pubkey, slot
        );
        if automation.next_instruction().is_some() {
            // If the automation has a next instruction, index it as executable.
            // let mut w_immediate_automations = self.immediate_automations.write().await;
            // w_immediate_automations.insert(automation_pubkey);
            // drop(w_immediate_automations);
        } else {
            // Otherwise, index the automation according to its trigger type.
            // match automation.trigger() {
            //     Trigger::Account {
            //         address,
            //         offset: _,
            //         size: _,
            //     } => {
            //         // Index the automation by its trigger's account pubkey.
            //         // let mut w_account_automations = self.account_automations.write().await;
            //         // w_account_automations
            //         //     .entry(address)
            //         //     .and_modify(|v| {
            //         //         v.insert(automation_pubkey);
            //         //     })
            //         //     .or_insert_with(|| {
            //         //         let mut v = HashSet::new();
            //         //         v.insert(automation_pubkey);
            //         //         v
            //         //     });
            //         // drop(w_account_automations);
            //     }
            //     Trigger::Cron {
            //         schedule,
            //         skippable: _,
            //     } => {
            //         // Find a reference timestamp for calculating the automation's upcoming target time.
            //         // let reference_timestamp = match automation.exec_context() {
            //         //     None => automation.created_at().unix_timestamp,
            //         //     Some(exec_context) => match exec_context.trigger_context {
            //         //         TriggerContext::Cron { started_at } => started_at,
            //         //         _ => {
            //         //             return Err(GeyserPluginError::Custom(
            //         //                 "Invalid exec context".into(),
            //         //             ))
            //         //         }
            //         //     },
            //         // };

            //         // // Index the automation to its target timestamp
            //         // match next_moment(reference_timestamp, schedule) {
            //         //     None => {} // The automation does not have any upcoming scheduled target time
            //         //     Some(target_timestamp) => {
            //         //         let mut w_cron_automations = self.cron_automations.write().await;
            //         //         w_cron_automations
            //         //             .entry(target_timestamp)
            //         //             .and_modify(|v| {
            //         //                 v.insert(automation_pubkey);
            //         //             })
            //         //             .or_insert_with(|| {
            //         //                 let mut v = HashSet::new();
            //         //                 v.insert(automation_pubkey);
            //         //                 v
            //         //             });
            //         //         drop(w_cron_automations);
            //         //     }
            //         // }
            //     }
            //     Trigger::Immediate => {
            //         let mut w_immediate_automations = self.immediate_automations.write().await;
            //         w_immediate_automations.insert(automation_pubkey);
            //         drop(w_immediate_automations);
            //     }
            // }
        }

        Ok(())
    }
}

impl Debug for AutomationObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "automation-observer")
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
