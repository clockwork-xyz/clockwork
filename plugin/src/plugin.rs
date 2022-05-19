use {
    crate::{
        bucket::Bucket, cache::QueueCache, client::RPCClient, config::Config as PluginConfig,
        filter,
    },
    bincode::deserialize,
    cronos_sdk::scheduler::state::{Queue, Task},
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
    cache: Option<Arc<Mutex<QueueCache>>>,
    bucket: Option<Arc<Mutex<Bucket>>>,
    rt: Option<Arc<Runtime>>,
    latest_clock_value: i64,
}

impl Debug for CronosPlugin {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct Runtime {
    rt: tokio::runtime::Runtime,
}

#[derive(Error, Debug)]
pub enum CronosPluginError {
    #[error("Error reading and/or writing to local cache. Error message: ({msg})")]
    CacheError { msg: String },

    #[error("Error deserializing queue data")]
    QueueAccountInfoError,

    #[error("Error deserializing sysvar clock data")]
    ClockAccountInfoError,

    #[error("Error in tokio runtime")]
    TokioRuntimeError,
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
        self.cache = Some(Arc::new(Mutex::new(QueueCache::new())));
        self.client = Some(Arc::new(Client::new(config.keypath, config.rpc_url)));

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name(self.name())
            .worker_threads(10)
            .max_blocking_threads(10)
            .build()
            .map_err(|_e| PluginError::Custom(Box::new(CronosPluginError::TokioRuntimeError)))
            .unwrap();

        self.rt = Some(Arc::new(Runtime { rt }));
        self.latest_clock_value = 0;

        Ok(())
    }

    fn on_unload(&mut self) {
        info!("Unloading plugin: {:?}", self.name());

        self.bucket = None;
        self.cache = None;
        self.client = None;
        self.rt = None;
        self.latest_clock_value = 0;
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
                    // Spawn an async tokio task for each lookback window
                    let clock = deserialize::<Clock>(info.data).map_err(|_| {
                        PluginError::Custom(Box::new(CronosPluginError::ClockAccountInfoError))
                    })?;
                    if self.latest_clock_value < clock.unix_timestamp {
                        self.latest_clock_value = clock.unix_timestamp;
                        let cp_arc: Arc<CronosPlugin> = Arc::new(self.clone());
                        let cp_clone = cp_arc.clone();
                        self.unwrap_rt().rt.spawn(async move {
                            cp_clone.process_queues_in_lookback_window().await;
                        });
                    }
                } else if &cronos_sdk::scheduler::ID.to_bytes() == info.owner {
                    // Cache the queue data
                    let key = Pubkey::new(info.pubkey);
                    let queue = Queue::try_from(info.data.to_vec()).map_err(|_| {
                        // TODO To optimize parallelization, should the data deserializtion be put into a tokio task?
                        //      Note, deserialization will will fail for all accounts that are not queues. How expensive
                        //      are deserialization errors?
                        PluginError::Custom(Box::new(CronosPluginError::QueueAccountInfoError))
                    })?;
                    let cp_arc: Arc<CronosPlugin> = Arc::new(self.clone());
                    let cp_clone = cp_arc.clone();
                    self.unwrap_rt()
                        .rt
                        .spawn(async move { cp_clone.write_to_cache(key, queue).await });
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
            rt: None,
            latest_clock_value: 0,
        }
    }
    fn unwrap_bucket(&self) -> &Arc<Mutex<Bucket>> {
        self.bucket.as_ref().expect("client is unavailable")
    }
    fn unwrap_cache(&self) -> &Arc<Mutex<QueueCache>> {
        self.cache.as_ref().expect("cache is unavailable")
    }
    fn unwrap_client(&self) -> &Arc<Client> {
        self.client.as_ref().expect("client is unavailable")
    }
    fn unwrap_rt(&self) -> &Arc<Runtime> {
        self.rt.as_ref().expect("tokio runtime is unavailable")
    }
    fn unwrap_update_account(account: ReplicaAccountInfoVersions) -> &ReplicaAccountInfo {
        match account {
            ReplicaAccountInfoVersions::V0_0_1(info) => info,
        }
    }

    async fn write_to_cache(&self, key: Pubkey, queue: Queue) {
        info!("Caching queue {}", key);
        info!("{:?}", queue);

        let mut cache = self.unwrap_cache().lock().await;

        match queue.exec_at {
            Some(_t) => cache.insert(key, queue),
            None => cache.delete(key),
        }
    }

    async fn process_queues_in_lookback_window(&self) {
        let cp_arc: Arc<CronosPlugin> = Arc::new(self.clone());

        const LOOKBACK_WINDOW: i64 = 7; // Number of seconds to lookback
        info!("Processing queues for ts {}", self.latest_clock_value);

        // Spawn tokio tasks to submit txns for the lookback window
        for t in (self.latest_clock_value - LOOKBACK_WINDOW)..=self.latest_clock_value {
            let cache = self.unwrap_cache().lock().await;
            cache.index.get(&t).and_then(|keys| {
                for key in keys.iter() {
                    cache.data.get(&key).and_then(|queue| {
                        let cp_clone = cp_arc.clone();
                        let key_clone = key.clone();
                        let queue_clone = queue.value().clone();

                        self.unwrap_rt().rt.spawn(async move {
                            cp_clone.process_queue(key_clone, queue_clone).await;
                        });

                        Some(())
                    });
                }
                Some(())
            });
        }
    }

    async fn process_queue(&self, queue_pubkey: Pubkey, queue: Queue) {
        let cp_arc: Arc<CronosPlugin> = Arc::new(self.clone());
        let cp_clone = cp_arc.clone();

        // Lock the mutex for this queue
        let mutex = cp_clone
            .unwrap_bucket()
            .lock()
            .await
            .get_mutex((queue_pubkey, queue.exec_at.unwrap()));

        let guard = mutex.try_lock();
        if guard.is_err() {
            return;
        };
        let guard = guard.unwrap();

        // Build task_begin ix
        let delegate_pubkey = cp_clone.unwrap_client().payer_pubkey();
        let queue_start_ix = cronos_sdk::scheduler::instruction::queue_start(
            delegate_pubkey,
            queue.manager,
            queue_pubkey,
        );

        // Accumulate task ixs here
        let mut ixs: Vec<Instruction> = vec![queue_start_ix];

        // Build an ix for each task
        for i in 0..queue.task_count {
            // Get the action account
            let task_pubkey = Task::pda(queue_pubkey, i).0;
            let task_data = cp_clone.unwrap_client().get_account_data(&task_pubkey);
            if task_data.is_err() {
                return;
            }
            let task_data = Task::try_from(task_data.unwrap()).unwrap();

            // Build ix
            let mut task_exec_ix = cronos_sdk::scheduler::instruction::task_exec(
                cp_clone.unwrap_client().payer_pubkey(),
                queue.manager,
                queue_pubkey,
                task_pubkey,
            );

            // Inject accounts for inner ixs
            let mut acc_dedupe = HashSet::<Pubkey>::new();
            for inner_ix in &task_data.ixs {
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

                        // Inject the delegate pubkey as the Cronos "payer" account
                        let mut payer_pubkey = acc.pubkey;
                        if acc.pubkey == cronos_sdk::scheduler::payer::ID {
                            payer_pubkey = delegate_pubkey;
                        }
                        task_exec_ix.accounts.push(match acc.is_writable {
                            true => AccountMeta::new(payer_pubkey, false),
                            false => AccountMeta::new_readonly(payer_pubkey, false),
                        })
                    }
                }
            }

            // Add to the list
            ixs.push(task_exec_ix)
        }

        // Sign and submit
        let res = cp_clone.unwrap_client().sign_and_submit(
            ixs.as_slice(),
            format!(
                "🤖 Processing queue {} for timestamp {}",
                queue_pubkey,
                queue.exec_at.unwrap()
            )
            .as_str(),
        );

        if res.is_err() {
            info!("❌ Failed to process queue: {:?}", res.err())
        }

        // Drop the mutex
        drop(guard)
    }
}
