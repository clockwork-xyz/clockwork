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
        collections::HashSet,
        fmt::{Debug, Formatter},
        sync::Arc,
    },
    thiserror::Error,
    tokio::sync::Mutex,
};

#[derive(Clone)]
pub struct CronosPlugin {
    client: Option<Arc<Client>>,
    cache: Option<Arc<Mutex<TaskCache>>>,
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
        self.cache = Some(Arc::new(Mutex::new(TaskCache::new())));
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

                                // cloning self because self needs to have a 'static lifetime
                                let self_clone = self.clone();
                                let cp_arc: Arc<CronosPlugin> = Arc::new(self_clone);
                                let cp_clone = cp_arc.clone();

                                // concurrently spawn task for each lookback window
                                tokio::spawn(async move {
                                    cp_clone.execute_tasks_in_lookback_window().await;
                                });
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
                            // cloning self because self needs to have a 'static lifetime
                            let self_clone = self.clone();
                            let cp_arc: Arc<CronosPlugin> = Arc::new(self_clone);
                            let cp_clone = cp_arc.clone();

                            tokio::spawn(async move { cp_clone.replicate_task(key, task).await });
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
    fn unwrap_cache(&self) -> &Arc<Mutex<TaskCache>> {
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

    async fn replicate_task(&self, key: Pubkey, task: Task) {
        info!("Caching task {}", key);
        info!("{:#?}", task);

        let mut cache = self.unwrap_cache().lock().await;

        match task.exec_at {
            Some(_t) => cache.insert(key, task),
            None => cache.delete(key),
        }
    }

    async fn execute_tasks_in_lookback_window(&self) {
        let self_clone = self.clone();
        let cp_arc: Arc<CronosPlugin> = Arc::new(self_clone);
        let cp_clone = cp_arc.clone();

        tokio::spawn(async move {
            const LOOKBACK_WINDOW: i64 = 7; // Number of seconds to lookback
            info!("Executing tasks for ts {}", cp_clone.latest_clock_value);

            // Spawn tokio tasks to execute cronos tasks in lookback window
            for t in (cp_clone.latest_clock_value - LOOKBACK_WINDOW)..=cp_clone.latest_clock_value {
                let cache = cp_clone.unwrap_cache().lock().await;
                cache.index.get(&t).and_then(|keys| {
                    for key in keys.iter() {
                        cache.data.get(&key).and_then(|task| {
                            let _ = async {
                                cp_clone.execute_task(*key, task.clone()).await;
                            };

                            Some(())
                        });
                    }
                    Some(())
                });
            }
        });
    }

    async fn execute_task(&self, task_pubkey: Pubkey, task: Task) {
        let self_clone = self.clone();
        let cp_arc: Arc<CronosPlugin> = Arc::new(self_clone);
        let cp_clone = cp_arc.clone();

        tokio::spawn(async move {
            // Lock the mutex for this task
            let mutex = cp_clone
                .unwrap_bucket()
                .lock()
                .await
                .get_mutex((task_pubkey, task.exec_at.unwrap()));

            let guard = mutex.try_lock();
            if guard.is_err() {
                return;
            };
            let guard = guard.unwrap();

            // Common pubkeys
            let config_pubkey = Config::pda().0;
            let delegate_pubkey = cp_clone.unwrap_client().payer_pubkey();
            let fee_pubkey = Fee::pda(task.queue).0;

            // Build task_begin ix
            let task_begin_ix = cronos_sdk::scheduler::instruction::task_begin(
                delegate_pubkey,
                task.queue,
                task_pubkey,
            );

            // Accumulate task ixs here
            let mut ixs: Vec<Instruction> = vec![task_begin_ix];

            // Build an ix for each action
            for i in 0..task.action_count {
                // Get the action account
                let action_pubkey = Action::pda(task_pubkey, i).0;
                let action_data = cp_clone.unwrap_client().get_account_data(&action_pubkey);
                if action_data.is_err() {
                    return;
                }
                let action_data = Action::try_from(action_data.unwrap()).unwrap();

                // Build ix
                let mut task_exec_ix = cronos_sdk::scheduler::instruction::task_exec(
                    action_pubkey,
                    config_pubkey,
                    cp_clone.unwrap_client().payer_pubkey(),
                    fee_pubkey,
                    task.queue,
                    task_pubkey,
                );

                // Inject accounts for inner ixs
                let mut acc_dedupe = HashSet::<Pubkey>::new();
                for inner_ix in &action_data.ixs {
                    // Program ids
                    if !acc_dedupe.contains(&inner_ix.program_id) {
                        acc_dedupe.insert(inner_ix.program_id);
                        task_exec_ix
                            .accounts
                            .push(AccountMeta::new_readonly(inner_ix.program_id, false));
                    }

                    // Other accounts
                    for acc in &inner_ix.accounts {
                        if !acc_dedupe.contains(&acc.pubkey) {
                            acc_dedupe.insert(acc.pubkey);

                            // Override the injected pubkey for the Cronos delegate account
                            let mut injected_pubkey = acc.pubkey;
                            if acc.pubkey == cronos_sdk::scheduler::delegate::ID {
                                injected_pubkey = delegate_pubkey;
                            }

                            // Push the account metadata into the ix as a "remaining account"
                            task_exec_ix.accounts.push(match acc.is_writable {
                                true => AccountMeta::new(injected_pubkey, false),
                                false => AccountMeta::new_readonly(injected_pubkey, false),
                            })
                        }
                    }
                }

                // Add to the list
                ixs.push(task_exec_ix)
            }

            // Sign and submit
            let res = cp_clone
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
                .await;

            if res.is_err() {
                info!("❌ Failed to execute task: {:#?}", res.err())
            }

            // Drop the mutex
            drop(guard)
        });
    }
}
