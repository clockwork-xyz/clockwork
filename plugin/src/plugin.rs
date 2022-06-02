use {
    crate::{config::PluginConfig, filter::CronosAccountUpdate},
    bugsnag::Bugsnag,
    cronos_client::{
        scheduler::state::{Queue, Task},
        Client,
    },
    dashmap::{DashMap, DashSet},
    log::info,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, Result as PluginResult,
        SlotStatus,
    },
    solana_program::{
        clock::Clock,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::signature::Signature,
    std::{collections::HashSet, fmt::Debug, sync::Arc},
    tokio::runtime::{Builder, Runtime},
};

#[derive(Debug, Default)]
pub struct CronosPlugin {
    inner: Option<Arc<Inner>>,
}

impl CronosPlugin {
    #[inline]
    fn with_inner<T>(&self, f: impl FnOnce(&Arc<Inner>) -> PluginResult<T>) -> PluginResult<T> {
        match self.inner {
            Some(ref inner) => f(inner).map_err(|e| GeyserPluginError::Custom(e.into())),
            None => Err(GeyserPluginError::Custom("Inner is uninitialized".into())),
        }
    }
}

impl GeyserPlugin for CronosPlugin {
    fn name(&self) -> &'static str {
        "cronos-plugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        solana_logger::setup_with_default("info");
        let config = PluginConfig::read_from(config_file)?;
        self.inner = Some(Arc::new(Inner {
            config: config.clone(),
            client: Client::new(config.keypath, config.rpc_url),
            runtime: Builder::new_multi_thread()
                .enable_all()
                .thread_name("cronos-plugin")
                .worker_threads(config.worker_threads)
                .max_blocking_threads(config.worker_threads)
                .build()
                .unwrap(),
            actionable_queues: DashSet::new(),
            pending_queues: DashMap::new(),
            signatures: DashMap::new(),
            timestamps: DashMap::new(),
        }));
        Ok(())
    }

    fn on_unload(&mut self) {
        self.inner = None;
    }

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        _slot: u64,
        _is_startup: bool,
    ) -> PluginResult<()> {
        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_1(account_info) => account_info.clone(),
        };
        let account_pubkey = Pubkey::new(account_info.clone().pubkey);

        match CronosAccountUpdate::try_from(account_info) {
            Ok(account_update) => {
                self.with_inner(|this| {
                    this.spawn(|this| async move {
                        match account_update {
                            CronosAccountUpdate::Clock { clock } => this.handle_clock_update(clock),
                            CronosAccountUpdate::Queue { queue } => {
                                info!("Caching queue {:#?}", account_pubkey);
                                this.handle_queue_update(queue, account_pubkey)
                            }
                        }
                    })
                })?;
            }
            Err(_) => (),
        };

        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> PluginResult<()> {
        Ok(())
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        _parent: Option<u64>,
        status: solana_geyser_plugin_interface::geyser_plugin_interface::SlotStatus,
    ) -> PluginResult<()> {
        match status {
            SlotStatus::Confirmed => self.with_inner(|this| {
                this.spawn(|this| async move { this.handle_confirmed_slot(slot) })
            }),
            _ => Ok(()),
        }
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

#[derive(Debug)]
pub struct Inner {
    pub client: Client,
    pub config: PluginConfig,
    pub runtime: Runtime,

    // The set of queues that can be processed
    pub actionable_queues: DashSet<Pubkey>,

    // Map from exec_at timestamps to the list of queues scheduled
    //  for that moment.
    pub pending_queues: DashMap<i64, DashSet<Pubkey>>,

    // Map from tx signatures to a (queue pubkey, slot) tuple. The slot
    //  represents the latest confirmed slot at the time the tx was sent.
    pub signatures: DashMap<Signature, (Pubkey, u64)>,

    // Map from slot numbers to sysvar clock unix_timestamps
    pub timestamps: DashMap<u64, i64>,
}

impl Inner {
    fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        info!("Confirmed slot: {}", confirmed_slot);

        // Look for the latest confirmed sysvar unix timestamp
        let mut confirmed_unix_timestamp = None;
        self.timestamps.retain(|slot, unix_timestamp| {
            if *slot == confirmed_slot {
                confirmed_unix_timestamp = Some(unix_timestamp.clone());
                return true;
            }
            *slot > confirmed_slot
        });

        // Move all pending queues that are due to the set of actionable queues.
        // TODO By maintaining a sorted list of the pending_queue's keys,
        //      this operation can possibly be made much cheaper. By iterating
        //      through the sorted list up to the confirmed unix timestamp, we
        //      save compute cycles by not iterating over future exec_at timestamps.
        //      However before doing this, consider if retain() can be processed in parallel...
        match confirmed_unix_timestamp {
            Some(confirmed_unix_timestamp) => {
                self.pending_queues.retain(|exec_at, queue_pubkeys| {
                    if *exec_at <= confirmed_unix_timestamp {
                        for queue_pubkey in queue_pubkeys.iter() {
                            self.actionable_queues.insert(queue_pubkey.clone());
                        }
                        return false;
                    }
                    true
                });
            }
            None => (),
        }

        // Process queues
        self.clone()
            .spawn(|this| async move { this.process_actionable_queues(confirmed_slot) })?;

        // Confirm signatures
        self.clone()
            .spawn(|this| async move { this.confirm_signatures(confirmed_slot) })?;

        Ok(())
    }

    fn handle_clock_update(self: Arc<Self>, clock: Clock) -> PluginResult<()> {
        self.timestamps.insert(clock.slot, clock.unix_timestamp);
        Ok(())
    }

    fn handle_queue_update(
        self: Arc<Self>,
        queue: Queue,
        queue_pubkey: Pubkey,
    ) -> PluginResult<()> {
        match queue.exec_at {
            Some(exec_at) => {
                self.pending_queues
                    .entry(exec_at)
                    .and_modify(|v| {
                        v.insert(queue_pubkey);
                    })
                    .or_insert_with(|| {
                        let v = DashSet::new();
                        v.insert(queue_pubkey);
                        v
                    });
            }
            None => (),
        };
        Ok(())
    }

    fn process_actionable_queues(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        // Error early if the node is not healthy
        self.client.get_health().map_err(|_| {
            info!("Node is not healthy");
            return GeyserPluginError::Custom("Node is not healthy".into());
        })?;

        // Async process each queue
        let actionable_queues = self.actionable_queues.iter();
        for queue_pubkey_ref in actionable_queues {
            let queue_pubkey = *queue_pubkey_ref.key();
            self.clone()
                .spawn(|this| async move { this.process_queue(queue_pubkey, confirmed_slot) })?;
        }
        Ok(())
    }

    fn process_queue(
        self: Arc<Self>,
        queue_pubkey: Pubkey,
        confirmed_slot: u64,
    ) -> PluginResult<()> {
        info!("Processing queue {}", queue_pubkey);

        // Get the queue
        let queue = self.client.get::<Queue>(&queue_pubkey).unwrap();

        // Build queue_start ix
        let delegate_pubkey = self.client.payer_pubkey();
        let queue_start_ix = cronos_client::scheduler::instruction::queue_start(
            delegate_pubkey,
            queue.manager,
            queue_pubkey,
        );

        // Accumulate task ixs here
        let mut ixs: Vec<Instruction> = vec![queue_start_ix];

        // Build an ix for each task
        for i in 0..queue.task_count {
            // Get the task account
            let task_pubkey = Task::pda(queue_pubkey, i).0;
            let task = self.client.get::<Task>(&task_pubkey).unwrap();

            // Build task_exec ix
            let mut task_exec_ix = cronos_client::scheduler::instruction::task_exec(
                delegate_pubkey,
                queue.manager,
                queue_pubkey,
                task_pubkey,
            );

            // Inject accounts for inner ixs
            let mut acc_dedupe = HashSet::<Pubkey>::new();
            for inner_ix in &task.ixs {
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
                        if acc.pubkey == cronos_client::scheduler::payer::ID {
                            payer_pubkey = delegate_pubkey;
                        }
                        task_exec_ix.accounts.push(match acc.is_writable {
                            true => AccountMeta::new(payer_pubkey, false),
                            false => AccountMeta::new_readonly(payer_pubkey, false),
                        })
                    }
                }
            }

            // Add ix to the list
            ixs.push(task_exec_ix)
        }

        // Pack all ixs into a single tx
        match self.client.send(ixs.as_slice(), &[self.client.payer()]) {
            Ok(signature) => {
                info!("✅ {}", signature);
                self.actionable_queues.remove(&queue_pubkey);
                self.signatures
                    .insert(signature, (queue_pubkey, confirmed_slot));
            }
            Err(err) => {
                info!("❌ {:#?}", err);
                self.actionable_queues.remove(&queue_pubkey);
                self.record_error(err.to_string())
            }
        }

        Ok(())
    }

    fn confirm_signatures(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        for signature_ref in self.signatures.iter() {
            let signature = *signature_ref.key();
            let tuple = signature_ref.value();
            let queue_pubkey = tuple.0;
            let attempted_slot = tuple.1;
            self.clone().spawn(|this| async move {
                this.confirm_signature(attempted_slot, confirmed_slot, queue_pubkey, signature)
            })?
        }
        Ok(())
    }

    fn confirm_signature(
        self: Arc<Self>,
        attempted_slot: u64,
        confirmed_slot: u64,
        queue_pubkey: Pubkey,
        signature: Signature,
    ) -> PluginResult<()> {
        match self
            .client
            .get_signature_status(&signature)
            .map_err(|_| GeyserPluginError::Custom("Failed to get confirmation status".into()))?
        {
            Some(res) => {
                match res {
                    Ok(()) => {
                        // This signature doesn't need to be checked again
                        self.signatures.remove(&signature);
                    }
                    Err(err) => {
                        // TODO Check the error. Should this request be retried?
                        info!("Transaction {} failed with error {}", signature, err);
                    }
                }
            }
            None => {
                // If many slots have passed since the tx was sent, then assume failure
                //  and move the pubkey back into the set of actionable queues.
                if confirmed_slot > attempted_slot + self.config.slot_timeout_threshold {
                    self.signatures.remove(&signature);
                    self.actionable_queues.insert(queue_pubkey);
                }
            }
        }
        Ok(())
    }

    fn spawn<F: std::future::Future<Output = PluginResult<()>> + Send + 'static>(
        self: &Arc<Self>,
        f: impl FnOnce(Arc<Self>) -> F,
    ) -> PluginResult<()> {
        self.runtime.spawn(f(Arc::clone(self)));
        Ok(())
    }

    fn record_error(self: Arc<Self>, err_msg: String) {
        match self.config.bugsnag_api_key.clone() {
            Some(api_key) => {
                let mut bugsnag_client = Bugsnag::new(&api_key, env!("CARGO_MANIFEST_DIR"));
                bugsnag_client.set_app_info(
                    Some(env!("CARGO_PKG_VERSION")),
                    Some("development"),
                    Some("rust"),
                );
                bugsnag_client
                    .notify("Error", &err_msg)
                    .severity(bugsnag::Severity::Error);
            }
            None => (),
        }
    }
}
