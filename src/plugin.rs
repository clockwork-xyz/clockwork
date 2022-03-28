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

use log::debug;

use {
    // crate::*,
    crate::{bucket::Bucket, cache::TaskCache, *},
    cronos_sdk::account::Task,
    log::info,
    simple_error::simple_error,
    solana_accountsdb_plugin_interface::accountsdb_plugin_interface::{
        AccountsDbPlugin, AccountsDbPluginError as PluginError, ReplicaAccountInfo,
        ReplicaAccountInfoVersions, Result as PluginResult, SlotStatus as PluginSlotStatus,
    },
    solana_program::pubkey::Pubkey,
    std::sync::Mutex,
    std::{
        fmt::{Debug, Formatter},
        sync::{Arc, RwLock},
    },
    thiserror::Error,
};

#[derive(Default)]
pub struct CronosPlugin {
    cache: Option<Arc<RwLock<TaskCache>>>,
    bucket: Option<Arc<Mutex<Bucket>>>,
    filter: Option<Filter>,
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
}

impl AccountsDbPlugin for CronosPlugin {
    fn name(&self) -> &'static str {
        "CronosPlugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        if self.cache.is_some() {
            let err = simple_error!("Cronos Plugin already Loaded");
            return Err(PluginError::Custom(Box::new(err)));
        }

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

        self.cache = Some(Arc::new(RwLock::new(TaskCache::new())));
        self.bucket = Some(Arc::new(Mutex::new(Bucket::new())));

        info!("Loaded Cronos Plugin");

        Ok(())
    }

    fn on_unload(&mut self) {
        info!("Unloading plugin: {:?}", self.name());

        self.cache = None;
        self.bucket = None;
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
            Some(cache) => {
                let task = Task::try_from(info.data.to_vec());
                let key = Pubkey::new(info.pubkey);
                let lamports = info.lamports;

                match task {
                    Err(err) => {
                        return Err(PluginError::Custom(Box::new(
                            CronosPluginError::TaskAccountInfoError,
                        )))
                    }
                    Ok(task) => {
                        self.replicate_task(key, task, lamports);
                    }
                }
            }
        }

        Ok(())
    }

    // fn update_slot_status(
    //     &mut self,
    //     slot: u64,
    //     parent: Option<u64>,
    //     status: PluginSlotStatus,
    // ) -> PluginResult<()> {
    //     info!("Updating slot {:?} with status {:?}", slot, status);
    // }

    // fn account_data_notifications_enabled(&self) -> bool {
    //     true
    // }

    // fn transaction_notifications_enabled(&self) -> bool {
    //     false
    // }
}

impl CronosPlugin {
    pub fn new() -> Self {
        Default::default()
    }

    fn unwrap_filter(&self) -> &Filter {
        self.filter.as_ref().expect("filter is unavailable")
    }

    fn unwrap_cache(&self) -> &Arc<RwLock<TaskCache>> {
        self.cache.as_ref().expect("cache is unavailable")
    }

    fn unwrap_update_account(account: ReplicaAccountInfoVersions) -> &ReplicaAccountInfo {
        match account {
            ReplicaAccountInfoVersions::V0_0_1(info) => info,
        }
    }

    fn replicate_task(&self, key: Pubkey, task: Task, lamports: u64) {
        info!("💽 Replicating task {}", key);
        let mut w_cache = self.unwrap_cache().write().unwrap();
        if lamports == 0 {
            w_cache.delete(key)
        } else {
            w_cache.insert(key, task)
        }
    }

    // TODO:
    // fn execute_task() {}
}
