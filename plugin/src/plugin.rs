use {
    crate::{config::Config as PluginConfig, filter::CronosAccountUpdate},
    cronos_sdk::{
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
            client: Client::new(config.keypath, config.rpc_url),
            runtime: Builder::new_multi_thread()
                .enable_all()
                .thread_name("cronos-plugin")
                .worker_threads(10) // TODO add to config
                .max_blocking_threads(10) // TODO add to config
                .build()
                .unwrap(),
            queues: DashMap::new(),
            unix_timestamps: DashMap::new(),
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
    pub runtime: Runtime,
    pub queues: DashMap<i64, DashSet<Pubkey>>,
    pub unix_timestamps: DashMap<u64, i64>,
}

impl Inner {
    fn handle_confirmed_slot(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        info!("Confirmed slot: {}", slot);

        // Remove cached timestamp for the prev slot
        let prev_slot = slot - 1;
        let last_ts = *self.unix_timestamps.get(&prev_slot).unwrap().value();
        self.unix_timestamps.remove(&prev_slot);

        // Process queues
        match self.clone().unix_timestamps.get(&slot) {
            Some(entry) => {
                if last_ts < *entry.value() {
                    self.process_queues_in_lookback_window(*entry.value())?;
                }
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn handle_clock_update(self: Arc<Self>, clock: Clock) -> PluginResult<()> {
        self.unix_timestamps
            .insert(clock.slot, clock.unix_timestamp);
        Ok(())
    }

    fn handle_queue_update(
        self: Arc<Self>,
        queue: Queue,
        queue_pubkey: Pubkey,
    ) -> PluginResult<()> {
        match queue.exec_at {
            Some(exec_at) => {
                self.queues
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

    fn process_queues_in_lookback_window(self: Arc<Self>, unix_timestamp: i64) -> PluginResult<()> {
        const LOOKBACK_WINDOW: i64 = 7; // Number of seconds to lookback
        for t in (unix_timestamp - LOOKBACK_WINDOW)..=unix_timestamp {
            match self.queues.get(&t) {
                Some(entry) => {
                    for queue_pubkey in entry.value().iter() {
                        let pk = queue_pubkey.clone();
                        self.clone()
                            .spawn(|this| async move { this.process_queue(pk) })?;
                    }
                }
                None => (),
            }
        }
        Ok(())
    }

    fn process_queue(self: Arc<Self>, queue_pubkey: Pubkey) -> PluginResult<()> {
        info!("Processing queue {}", queue_pubkey);

        // Get the queue
        let queue = self.client.get::<Queue>(&queue_pubkey).unwrap();

        // Build queue_start ix
        let delegate_pubkey = self.client.payer_pubkey();
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
            let task = self.client.get::<Task>(&task_pubkey).unwrap();

            // Build ix
            let mut task_exec_ix = cronos_sdk::scheduler::instruction::task_exec(
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

        // Pack all ixs into a single tx
        match self
            .client
            .sign_and_submit(ixs.as_slice(), &[self.client.payer()])
        {
            Ok(sig) => {
                info!("✅ {}", sig);
                self.queues.entry(queue.exec_at.unwrap()).and_modify(|v| {
                    v.remove(&queue_pubkey);
                });
            }
            Err(err) => info!("❌ {:#?}", err),
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
}
