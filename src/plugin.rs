// Copyright 2022 Blockdaemon Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{borrow::BorrowMut, future::Future, sync::RwLockWriteGuard};

use solana_account_decoder::parse_sysvar::{parse_sysvar, SysvarAccountType};
use solana_program::{
    clock::{Clock, Epoch, Slot, UnixTimestamp},
    sysvar::{self, Sysvar},
};
use solana_sdk::account::Account;

use crate::env;

use {
    crate::{bucket::Bucket, cache::TaskCache, client::RPCClient, Config, Filter},
    bincode::deserialize,
    cronos_program::state::{Config as cpConfig, Fee},
    cronos_sdk::account::{Task, TaskStatus},
    log::debug,
    log::info,
    solana_account_decoder::parse_sysvar,
    solana_accountsdb_plugin_interface::accountsdb_plugin_interface::{
        AccountsDbPlugin, AccountsDbPluginError as PluginError, ReplicaAccountInfo,
        ReplicaAccountInfoVersions, Result as PluginResult,
    },
    solana_client_helpers::Client,
    solana_program::pubkey::Pubkey,
    solana_sdk::instruction::AccountMeta,
    std::sync::Mutex,
    std::{
        fmt::{Debug, Formatter},
        sync::{Arc, RwLock},
        thread,
    },
    thiserror::Error,
};
pub struct CronosPlugin {
    client: Option<Arc<Client>>,
    cache: Option<Arc<RwLock<TaskCache>>>,
    bucket: Option<Arc<Mutex<Bucket>>>,
    filter: Option<Filter>,
    latest_clock_value: i64,
}

impl Debug for CronosPlugin {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum CronosPluginError {
    #[error("Error writing to local cache. Error message: ({msg})")]
    CacheError { msg: String },

    #[error("Error deserializing task data")]
    TaskAccountInfoError,

    #[error("Error deserializing sysvar clock data")]
    ClockAccountInfoError,
}

impl AccountsDbPlugin for CronosPlugin {
    fn name(&self) -> &'static str {
        "CronosPlugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        solana_logger::setup_with_default("info");

        info!(
            "Loading plugin {:?} from config_file {:?}",
            self.name(),
            config_file
        );

        let result = Config::read_from(config_file);

        match result {
            Err(err) => {
                return Err(PluginError::ConfigFileReadError {
                    msg: format!(
                        "The config file is not in the JSON format expected: {:?}",
                        err
                    ),
                })
            }
            Ok(config) => self.filter = Some(Filter::new(&config)),
        }

        self.bucket = Some(Arc::new(Mutex::new(Bucket::new())));
        self.cache = Some(Arc::new(RwLock::new(TaskCache::new())));
        self.client = Some(Arc::new(Client::new()));
        self.latest_clock_value = 0;

        info!("Loaded Cronos Plugin");

        Ok(())
    }

    fn on_unload(&mut self) {
        info!("Unloading plugin: {:?}", self.name());

        self.bucket = None;
        self.cache = None;
        self.client = None;
        self.filter = None;
    }

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        if is_startup {
            return Ok(());
        }

        let info = Self::unwrap_update_account(account);

        if !self.unwrap_filter().wants_program(info.owner) {
            return Ok(());
        }

        debug!(
            "Updating account {:?} with owner {:?} at slot {:?}",
            info.pubkey, info.owner, slot
        );

        match &mut self.cache {
            None => {
                return Err(PluginError::Custom(Box::new(
                    CronosPluginError::CacheError {
                        msg: "There is no available cache to update account data".to_string(),
                    },
                )));
            }
            Some(_cache) => {
                if &sysvar::clock::id().to_bytes() == info.pubkey {
                    let clock = deserialize::<Clock>(info.data);

                    match clock {
                        Err(_err) => {
                            return Err(PluginError::Custom(Box::new(
                                CronosPluginError::ClockAccountInfoError,
                            )))
                        }
                        Ok(clock) => {
                            if self.latest_clock_value < clock.unix_timestamp {
                                self.latest_clock_value = clock.unix_timestamp;
                                info!("clock unix_timestamp: {}", self.latest_clock_value);
                                self.execute_tasks_in_lookback_window();
                            }
                        }
                    }

                //TODO: better check for validating cronos related accounts
                } else if &sysvar::id().to_bytes() != info.owner {
                    info!("cronos here!");

                    let task = Task::try_from(info.data.to_vec());
                    let key = Pubkey::new(info.pubkey);

                    match task {
                        Err(_err) => {
                            return Err(PluginError::Custom(Box::new(
                                CronosPluginError::TaskAccountInfoError,
                            )))
                        }
                        Ok(task) => {
                            self.replicate_task(key, task);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> PluginResult<()> {
        Ok(())
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        parent: Option<u64>,
        status: solana_accountsdb_plugin_interface::accountsdb_plugin_interface::SlotStatus,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_transaction(
        &mut self,
        transaction: solana_accountsdb_plugin_interface::accountsdb_plugin_interface::ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_block_metadata(
        &mut self,
        blockinfo: solana_accountsdb_plugin_interface::accountsdb_plugin_interface::ReplicaBlockInfoVersions,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false
    }
}

impl CronosPlugin {
    pub fn new() -> Self {
        Self {
            client: Some(Arc::new(Client::new())),
            cache: Some(Arc::new(RwLock::new(TaskCache::new()))),
            bucket: Some(Arc::new(Mutex::new(Bucket::new()))),
            filter: None,
            latest_clock_value: 0,
        }
    }
    fn unwrap_bucket(&self) -> &Arc<Mutex<Bucket>> {
        self.bucket.as_ref().expect("client is unavailable")
    }
    fn unwrap_cache(&self) -> &Arc<RwLock<TaskCache>> {
        self.cache.as_ref().expect("cache is unavailable")
    }
    fn unwrap_client(&self) -> &Arc<Client> {
        self.client.as_ref().expect("client is unavailable")
    }
    fn unwrap_filter(&self) -> &Filter {
        self.filter.as_ref().expect("filter is unavailable")
    }
    fn unwrap_update_account(account: ReplicaAccountInfoVersions) -> &ReplicaAccountInfo {
        match account {
            ReplicaAccountInfoVersions::V0_0_1(info) => info,
        }
    }

    fn replicate_task(&self, key: Pubkey, task: Task) {
        info!("💽 Replicating task {}", key);
        let mut w_cache = self.unwrap_cache().write().unwrap();
        match task.status {
            TaskStatus::Queued => w_cache.insert(key, task),
            TaskStatus::Cancelled | TaskStatus::Done => w_cache.delete(key),
        }
    }

    fn execute_tasks_in_lookback_window(&self) {
        // thread::spawn(move || {
        const LOOKBACK_WINDOW: i64 = 60 * 15; // Number of seconds to lookback
        info!("executing tasks for unix_ts: {}", self.latest_clock_value);

        // Spawn threads to execute tasks in lookback window
        // let mut handles = vec![];
        for t in (self.latest_clock_value - LOOKBACK_WINDOW)..=self.latest_clock_value {
            let r_cache = self.unwrap_cache().read().unwrap();
            r_cache.index.get(&t).and_then(|keys| {
                for key in keys.iter() {
                    r_cache.data.get(key).and_then(|task| {
                        // handles.push(self.execute_task(*key, task.clone()));
                        self.execute_task(*key, task.clone());
                        Some(())
                    });
                }
                Some(())
            });
        }

        // Join threads
        // if !handles.is_empty() {
        //     for h in handles {
        //         h.join().unwrap();
        //     }
        // }
        // });
    }

    fn execute_task(&self, key: Pubkey, task: Task) -> () {
        // thread::spawn(move || {
        // Lock the mutex for this task
        let mutex = self
            .unwrap_bucket()
            .lock()
            .unwrap()
            .get_mutex((key, task.schedule.exec_at));
        let guard = mutex.try_lock();
        if guard.is_err() {
            return;
        };
        // let guard = guard.unwrap();

        let data = self
            .unwrap_client()
            .client
            .get_account_data(&task.daemon)
            .unwrap();

        let daemon_data = cronos_sdk::account::Daemon::try_from(data).unwrap();

        info!(
            "daemon data: owner - {}, task count - {}",
            daemon_data.owner, daemon_data.task_count
        );

        // Get accounts
        let config = cpConfig::pda().0;
        let fee = Fee::pda(task.daemon).0;

        // for acc in &task.ix.accounts {
        //     let acc_info = self.unwrap_client().client.get_account(&acc.pubkey);

        //     if acc_info.is_err() {
        //         info!("address {} does not exist", &acc.pubkey);
        //     }
        // }

        // Add accounts to exec instruction
        let mut ix_exec = cronos_sdk::instruction::task_execute(
            config,
            task.daemon,
            fee,
            key,
            self.unwrap_client().payer_pubkey(),
        );
        for acc in task.ix.accounts {
            match acc.is_writable {
                true => ix_exec.accounts.push(AccountMeta::new(acc.pubkey, false)),
                false => ix_exec
                    .accounts
                    .push(AccountMeta::new_readonly(acc.pubkey, false)),
            }
        }
        ix_exec
            .accounts
            .push(AccountMeta::new_readonly(task.ix.program_id, false));

        // Sign and submit
        let res = self.unwrap_client().sign_and_submit(
            &[ix_exec],
            format!("🤖 Executing task: {} {}", key, task.schedule.exec_at).as_str(),
        );

        // If exec failed, replicate the task data
        if res.is_err() {
            let err = res.err().unwrap();
            info!("❌ {}", err);
            // let data = self.unwrap_client().get_account_data(&key).unwrap();
            // let task = Task::try_from(data).unwrap();
            // let mut w_cache = self.unwrap_cache().write().unwrap();
            // w_cache.insert(key, task);
        }

        // Drop the mutex
        // drop(guard)
        // })
    }
}
