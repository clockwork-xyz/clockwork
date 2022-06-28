use {
    crate::{
        config::PluginConfig, delegate::Delegate, events::AccountUpdateEvent, scheduler::Scheduler,
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
    pub delegate: Arc<Delegate>,
    pub scheduler: Arc<Scheduler>,
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
        self.runtime = build_runtime(config.clone());
        self.delegate = Arc::new(Delegate::new(config.clone(), self.runtime.clone()));
        self.scheduler = Arc::new(Scheduler::new(
            config,
            self.delegate.pool_positions.clone(),
            self.runtime.clone(),
        ));
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
                    self.scheduler.clone().handle_updated_clock(clock)
                }
                AccountUpdateEvent::Pool { pool } => {
                    self.delegate.clone().handle_updated_pool(pool, slot)
                }
                AccountUpdateEvent::Queue { queue } => self
                    .scheduler
                    .clone()
                    .handle_updated_queue(queue, account_pubkey),
                AccountUpdateEvent::Rotator { rotator } => {
                    self.delegate.clone().handle_updated_rotator(rotator)
                }
                AccountUpdateEvent::Snapshot { snapshot } => {
                    self.delegate.clone().handle_updated_snapshot(snapshot)
                }
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
                self.delegate.clone().handle_confirmed_slot(slot)?;
                self.scheduler.clone().handle_confirmed_slot(slot)?;
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
        let runtime = build_runtime(config.clone());
        let delegate = Arc::new(Delegate::new(config.clone(), runtime.clone()));
        let scheduler = Arc::new(Scheduler::new(
            config.clone(),
            delegate.pool_positions.clone(),
            runtime.clone(),
        ));
        CronosPlugin {
            delegate,
            scheduler,
            runtime,
        }
    }
}

fn build_runtime(config: PluginConfig) -> Arc<Runtime> {
    Arc::new(
        Builder::new_multi_thread()
            .enable_all()
            .thread_name("cronos-executor")
            .worker_threads(config.worker_threads)
            .max_blocking_threads(config.worker_threads)
            .build()
            .unwrap(),
    )
}
