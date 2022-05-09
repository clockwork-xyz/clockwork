use std::collections::HashSet;

use {
    crate::{
        bucket::Bucket, cache::TaskCache, client::RPCClient, config::Config as PluginConfig, filter,
    },
    bincode::deserialize,
    cronos_sdk::scheduler::state::{Action, Config, Fee, Task},
    log::{debug, info},
    solana_client_helpers::Client,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, GeyserPluginError as PluginError, ReplicaAccountInfo,
        ReplicaAccountInfoVersions, Result as PluginResult,
    },
    solana_program::{
        clock::Clock,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    std::{
        fmt::{Debug, Formatter},
        sync::Mutex,
        sync::{Arc, RwLock},
        thread::{self, JoinHandle},
    },
    thiserror::Error,
};

#[derive(Clone)]
pub struct CronosPlugin {
    client: Option<Arc<Client>>,
    cache: Option<Arc<RwLock<TaskCache>>>,
    bucket: Option<Arc<Mutex<Bucket>>>,
    latest_clock_value: i64,
}

impl Debug for CronosPlugin {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum CronosPluginError {
    #[error("Error reading and/or writing to local cache. Error message: ({msg})")]
    CacheError { msg: String },

    #[error("Error deserializing task data")]
    TaskAccountInfoError,

    #[error("Error deserializing sysvar clock data")]
    ClockAccountInfoError,
}

impl GeyserPlugin for CronosPlugin {
    fn name(&self) -> &'static str {
        "CronosPlugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        solana_logger::setup_with_default("info");

        info!("Loading plugin {:?}", self.name());

        let config = PluginConfig::read_from(config_file)?;
        self.bucket = Some(Arc::new(Mutex::new(Bucket::new())));
        self.cache = Some(Arc::new(RwLock::new(TaskCache::new())));
        self.client = Some(Arc::new(Client::new(config.keypath, config.rpc_url)));
        self.latest_clock_value = 0;
        Ok(())
    }

    fn on_unload(&mut self) {
        info!("Unloading plugin: {:?}", self.name());

        self.bucket = None;
        self.cache = None;
        self.client = None;
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
        if !filter::wants_account(info) {
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
                                self.execute_tasks_in_lookback_window();
                            }
                        }
                    }
                } else if &cronos_sdk::scheduler::ID.to_bytes() == info.owner {
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
        _slot: u64,
        _parent: Option<u64>,
        _status: solana_geyser_plugin_interface::geyser_plugin_interface::SlotStatus,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_transaction(
        &mut self,
        _transaction: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions,
        _slot: u64,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_block_metadata(
        &mut self,
        _blockinfo: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaBlockInfoVersions,
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
            cache: None,
            client: None,
            bucket: None,
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
    fn unwrap_update_account(account: ReplicaAccountInfoVersions) -> &ReplicaAccountInfo {
        match account {
            ReplicaAccountInfoVersions::V0_0_1(info) => info,
        }
    }

    fn replicate_task(&self, key: Pubkey, task: Task) {
        info!("Caching task {}", key);
        let mut w_cache = self.unwrap_cache().write().unwrap();
        match task.exec_at {
            Some(_t) => w_cache.insert(key, task),
            None => w_cache.delete(key),
        }
    }

    fn execute_tasks_in_lookback_window(&self) {
        // Spawn threads to execute tasks in lookback window
        info!("Executing tasks for ts {}", self.latest_clock_value);
        const LOOKBACK_WINDOW: i64 = 2; // sec
        let mut handles = vec![];
        for t in (self.latest_clock_value - LOOKBACK_WINDOW)..=self.latest_clock_value {
            let r_cache = self.unwrap_cache().read().unwrap();
            r_cache.index.get(&t).and_then(|keys| {
                for key in keys.iter() {
                    r_cache.data.get(key).and_then(|task| {
                        handles.push(self.execute_task(*key, task.clone()));
                        Some(())
                    });
                }
                Some(())
            });
        }

        // Join threads
        if !handles.is_empty() {
            for h in handles {
                h.join().unwrap();
            }
        }
    }

    fn execute_task(&self, task_pubkey: Pubkey, task: Task) -> JoinHandle<()> {
        let self_clone = self.clone();
        let cp_arc: Arc<CronosPlugin> = Arc::new(self_clone);
        let cp_clone = cp_arc.clone();

        thread::spawn(move || {
            // Lock the mutex for this task
            let mutex = cp_clone
                .unwrap_bucket()
                .lock()
                .unwrap()
                .get_mutex((task_pubkey, task.exec_at.unwrap()));
            let guard = mutex.try_lock();
            if guard.is_err() {
                return;
            };
            let guard = guard.unwrap();

            // Common pubkeys
            let config = Config::pda().0;
            let fee = Fee::pda(task.queue).0;

            // Accumulate task_exec ixs here
            let mut ixs: Vec<Instruction> = vec![];

            // For each action...
            for i in 0..task.actions_count {
                // Get the action account
                let action_pubkey = Action::pda(task_pubkey, i).0;
                let action_data = Action::try_from(
                    cp_clone
                        .unwrap_client()
                        .get_account_data(&action_pubkey)
                        .unwrap(),
                )
                .unwrap();

                // Build ix
                let mut ix = cronos_sdk::scheduler::instruction::task_exec(
                    action_pubkey,
                    cp_clone.unwrap_client().payer_pubkey(),
                    config,
                    fee,
                    task.queue,
                    task_pubkey,
                );

                // Inject accounts for action inner ixs
                let mut acc_dedupe = HashSet::<Pubkey>::new();
                for inner_ix in &action_data.ixs {
                    // Program ids
                    if !acc_dedupe.contains(&inner_ix.program_id) {
                        acc_dedupe.insert(inner_ix.program_id);
                        ix.accounts
                            .push(AccountMeta::new_readonly(inner_ix.program_id, false));
                    }

                    // Other accounts
                    for acc in &inner_ix.accounts {
                        if !acc_dedupe.contains(&acc.pubkey) {
                            acc_dedupe.insert(acc.pubkey);
                            ix.accounts.push(match acc.is_writable {
                                true => AccountMeta::new(acc.pubkey, false),
                                false => AccountMeta::new_readonly(acc.pubkey, false),
                            })
                        }
                    }
                }

                ixs.push(ix)
            }

            // Sign and submit
            if ixs.is_empty() {
                info!("📂 No actions for task {}", task_pubkey);
            } else {
                cp_clone
                    .unwrap_client()
                    .sign_and_submit(
                        ixs.as_slice(),
                        format!(
                            "🤖 Executing task {} for timestamp {}",
                            task_pubkey,
                            task.exec_at.unwrap()
                        )
                        .as_str(),
                    )
                    .unwrap();
            }

            // Drop the mutex
            drop(guard)
        })
    }
}
