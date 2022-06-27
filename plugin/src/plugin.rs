use {
    crate::{
        config::PluginConfig,
        cycle_executor::{CycleExecutor, DelegateStatus},
        events::AccountUpdateEvent,
        task_executor::TaskExecutor,
    },
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, ReplicaAccountInfoVersions, Result as PluginResult, SlotStatus,
    },
    solana_program::pubkey::Pubkey,
    std::{fmt::Debug, sync::Arc},
    tokio::runtime::{Builder, Runtime},
};

#[derive(Debug)]
pub struct CronosPlugin {
    pub cycle_executor: Arc<CycleExecutor>,
    pub task_executor: Arc<TaskExecutor>,
    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl GeyserPlugin for CronosPlugin {
    fn name(&self) -> &'static str {
        "cronos-plugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        solana_logger::setup_with_default("info");
        let config = PluginConfig::read_from(config_file)?;
        let runtime = Arc::new(
            Builder::new_multi_thread()
                .enable_all()
                .thread_name("cronos-executor")
                .worker_threads(config.worker_threads)
                .max_blocking_threads(config.worker_threads)
                .build()
                .unwrap(),
        );
        let (sender, _receiver) = crossbeam::channel::unbounded::<DelegateStatus>();
        self.cycle_executor = Arc::new(CycleExecutor::new(config.clone(), runtime.clone(), sender));
        self.task_executor = Arc::new(TaskExecutor::new(config));
        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        _is_startup: bool,
    ) -> PluginResult<()> {
        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_1(account_info) => account_info.clone(),
        };
        let account_pubkey = Pubkey::new(account_info.clone().pubkey);

        match AccountUpdateEvent::try_from(account_info) {
            Ok(event) => match event {
                AccountUpdateEvent::Clock { clock } => {
                    self.task_executor.clone().handle_updated_clock(clock)
                }
                AccountUpdateEvent::Cycler { cycler } => {
                    self.cycle_executor.clone().handle_updated_cycler(cycler)
                }
                AccountUpdateEvent::Pool { pool } => {
                    self.cycle_executor.clone().handle_updated_pool(pool, slot)
                }
                AccountUpdateEvent::Queue { queue } => self
                    .task_executor
                    .clone()
                    .handle_updated_queue(queue, account_pubkey),
                AccountUpdateEvent::Snapshot { snapshot } => self
                    .cycle_executor
                    .clone()
                    .handle_updated_snapshot(snapshot),
            },
            Err(_err) => Ok(()),
        }
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
            SlotStatus::Confirmed => {
                self.cycle_executor.clone().handle_confirmed_slot(slot)?;
                self.task_executor.clone().handle_confirmed_slot(slot)?;
            }
            _ => (),
        }
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

impl Default for CronosPlugin {
    fn default() -> Self {
        let config = PluginConfig::default();
        let runtime = Arc::new(
            Builder::new_multi_thread()
                .enable_all()
                .thread_name("cronos-executor")
                .worker_threads(config.worker_threads)
                .max_blocking_threads(config.worker_threads)
                .build()
                .unwrap(),
        );
        let (sender, _receiver) = crossbeam::channel::unbounded::<DelegateStatus>();
        CronosPlugin {
            cycle_executor: Arc::new(CycleExecutor::new(config, runtime.clone(), sender)),
            task_executor: Arc::new(TaskExecutor::default()),
            runtime,
            // cycler:
        }
    }
}
