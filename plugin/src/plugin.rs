use {
    crate::{config::PluginConfig, events::AccountUpdateEvent, executor::Executor},
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, ReplicaAccountInfoVersions, Result as PluginResult, SlotStatus,
    },
    solana_program::pubkey::Pubkey,
    std::{fmt::Debug, sync::Arc},
};

#[derive(Debug)]
pub struct CronosPlugin {
    executor: Arc<Executor>,
}

impl GeyserPlugin for CronosPlugin {
    fn name(&self) -> &'static str {
        "cronos-plugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        solana_logger::setup_with_default("info");
        let config = PluginConfig::read_from(config_file)?;
        self.executor = Arc::new(Executor::new(config));
        Ok(())
    }

    fn on_unload(&mut self) {}

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

        match AccountUpdateEvent::try_from(account_info) {
            Ok(event) => match event {
                AccountUpdateEvent::Clock { clock } => {
                    self.executor.clone().handle_updated_clock(clock)
                }
                AccountUpdateEvent::Queue { queue } => self
                    .executor
                    .clone()
                    .handle_updated_queue(queue, account_pubkey),
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
            SlotStatus::Confirmed => self.executor.clone().handle_confirmed_slot(slot),
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

// impl CronosPlugin {
//     pub fn handle_updated_account(
//         self: &Self,
//         account_pubkey: Pubkey,
//         account_info: ReplicaAccountInfo,
//     ) -> PluginResult<()> {
//         AccountUpdateEvent::try_from(account_info).and_then(|event| match event {
//             AccountUpdateEvent::Clock { clock } => {
//                 self.executor.clone().handle_updated_clock(clock)
//             }
//             AccountUpdateEvent::Queue { queue } => self
//                 .executor
//                 .clone()
//                 .handle_updated_queue(queue, account_pubkey),
//         })
//     }
// }

impl Default for CronosPlugin {
    fn default() -> Self {
        CronosPlugin {
            executor: Arc::new(Executor::default()),
        }
    }
}
