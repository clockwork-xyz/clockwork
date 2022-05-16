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
        sync::Mutex,
        sync::{Arc, RwLock},
        thread::{self, JoinHandle},
    },
    thiserror::Error,
};

#[derive(Clone)]
pub struct CronosPlugin {
    client: Option<Arc<Client>>,
    cache: Option<Arc<RwLock<QueueCache>>>,
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

    #[error("Error deserializing queue data")]
    QueueAccountInfoError,

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
        self.cache = Some(Arc::new(RwLock::new(QueueCache::new())));
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
                                self.process_queues_in_lookback_window();
                            }
                        }
                    }
                } else if &cronos_sdk::scheduler::ID.to_bytes() == info.owner {
                    let queue = Queue::try_from(info.data.to_vec());
                    let key = Pubkey::new(info.pubkey);

                    match queue {
                        Err(_err) => {
                            return Err(PluginError::Custom(Box::new(
                                CronosPluginError::QueueAccountInfoError,
                            )))
                        }
                        Ok(queue) => {
                            self.replicate_queue(key, queue);
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
    fn unwrap_cache(&self) -> &Arc<RwLock<QueueCache>> {
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

    fn replicate_queue(&self, key: Pubkey, queue: Queue) {
        info!("Caching queue {}", key);
        info!("{:#?}", queue);

        let mut w_cache = self.unwrap_cache().write().unwrap();
        match queue.exec_at {
            Some(_t) => w_cache.insert(key, queue),
            None => w_cache.delete(key),
        }
    }

    fn process_queues_in_lookback_window(&self) {
        let self_clone = self.clone();
        let cp_arc: Arc<CronosPlugin> = Arc::new(self_clone);
        let cp_clone = cp_arc.clone();

        thread::spawn(move || {
            const LOOKBACK_WINDOW: i64 = 7; // Number of seconds to lookback
            info!("Processing queues for ts {}", cp_clone.latest_clock_value);

            // Spawn threads to process queues in lookback window
            let mut handles = vec![];
            for t in (cp_clone.latest_clock_value - LOOKBACK_WINDOW)..=cp_clone.latest_clock_value {
                let r_cache = cp_clone.unwrap_cache().read().unwrap();
                r_cache.index.get(&t).and_then(|keys| {
                    for key in keys.iter() {
                        r_cache.data.get(key).and_then(|queue| {
                            handles.push(cp_clone.process_queue(*key, queue.clone()));
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
        });
    }

    fn process_queue(&self, queue_pubkey: Pubkey, queue: Queue) -> JoinHandle<()> {
        let self_clone = self.clone();
        let cp_arc: Arc<CronosPlugin> = Arc::new(self_clone);
        let cp_clone = cp_arc.clone();

        thread::spawn(move || {
            // Lock the mutex for this queue
            let mutex = cp_clone
                .unwrap_bucket()
                .lock()
                .unwrap()
                .get_mutex((queue_pubkey, queue.exec_at.unwrap()));
            let guard = mutex.try_lock();
            if guard.is_err() {
                return;
            };
            let guard = guard.unwrap();

            // Common pubkeys
            let delegate_pubkey = cp_clone.unwrap_client().payer_pubkey();

            // Build queue_start ix
            let queue_start_ix = cronos_sdk::scheduler::instruction::queue_start(
                delegate_pubkey,
                queue.manager,
                queue_pubkey,
            );

            // Accumulate queue ixs here
            let mut ixs: Vec<Instruction> = vec![queue_start_ix];

            // Build an ix for each task
            for i in 0..queue.task_count {
                // Get the task account
                let task_pubkey = Task::pda(queue_pubkey, i).0;
                let task_data = cp_clone.unwrap_client().get_account_data(&task_pubkey);
                if task_data.is_err() {
                    return;
                }
                let task_data = Task::try_from(task_data.unwrap()).unwrap();

                // Build ix
                let mut task_exec_ix = cronos_sdk::scheduler::instruction::task_exec(
                    delegate_pubkey,
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

                            // Override the injected pubkey for the Cronos delegate account
                            let mut injected_pubkey = acc.pubkey;
                            if acc.pubkey == cronos_sdk::scheduler::payer::ID {
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
            let res = cp_clone.unwrap_client().sign_and_submit(
                ixs.as_slice(),
                format!(
                    "🤖 Executing task {} for timestamp {}",
                    queue_pubkey,
                    queue.exec_at.unwrap()
                )
                .as_str(),
            );
            if res.is_err() {
                info!("❌ Failed to execute task: {:#?}", res.err())
            }

            // Drop the mutex
            drop(guard)
        })
    }
}
