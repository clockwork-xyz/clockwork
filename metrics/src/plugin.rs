use std::{fmt::Debug, sync::Arc};

use anchor_lang::Discriminator;
use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfo, ReplicaAccountInfoVersions, ReplicaTransactionInfoV2,
    Result as PluginResult, SlotStatus,
};
use solana_program::pubkey::Pubkey;
use tokio::runtime::{Builder, Runtime};

use crate::events::AccountUpdateEvent;

pub struct ClockworkMetricsPlugin {
    pub inner: Arc<Inner>,
}

impl Debug for ClockworkMetricsPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "inner: {:?}", self.inner)
    }
}

#[derive(Debug)]
pub struct Inner {
    pub runtime: Arc<Runtime>,
}

impl GeyserPlugin for ClockworkMetricsPlugin {
    fn name(&self) -> &'static str {
        "clockwork-plugin"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        solana_logger::setup_with_default("info");
        info!(
            "clockwork-plugin crate-info - spec: {}, geyser_interface_version: {}, rustc: {}",
            env!("SPEC"),
            env!("GEYSER_INTERFACE_VERSION"),
            env!("RUSTC_VERSION")
        );
        info!("Loading snapshot...");
        *self = ClockworkMetricsPlugin::default();
        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &mut self,
        _account: ReplicaAccountInfoVersions,
        _slot: u64,
        _is_startup: bool,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> PluginResult<()> {
        Ok(())
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        _parent: Option<u64>,
        status: SlotStatus,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_transaction(
        &mut self,
        transaction: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> PluginResult<()> {
        let tx_info = match transaction {
            solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions::V0_0_1(t) => {
                ReplicaTransactionInfoV2 {
                    signature: t.signature,
                    is_vote: t.is_vote,
                    transaction: t.transaction,
                    transaction_status_meta: t.transaction_status_meta,
                    index: 0,
                }
            },
            solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions::V0_0_2(t) => {
                ReplicaTransactionInfoV2 {
                    signature: t.signature,
                    is_vote: t.is_vote,
                    transaction: t.transaction,
                    transaction_status_meta: t.transaction_status_meta,
                    index: t.index,
                }
            },
        };

        let is_clockwork_tx = if tx_info.is_vote {
            false
        } else {
            match tx_info.transaction.message() {
                solana_program::message::SanitizedMessage::Legacy(msg) => {
                    let program_ids = msg.message.program_ids();
                    if program_ids.contains(&&clockwork_thread_program_v1::ID)
                        || program_ids.contains(&&clockwork_thread_program::ID)
                    {
                        true
                    } else {
                        false
                    }
                }
                solana_program::message::SanitizedMessage::V0(_) => false,
            }
        };

        if is_clockwork_tx {
            let tx_sig = tx_info.signature;
            // let tx_status = tx_info.transaction_status_meta.status.is_ok();
            // let tx_err = tx_info.transaction_status_meta.status.unwrap_err;
            for (program_id, _ix) in tx_info.transaction.message().program_instructions_iter() {
                if program_id.eq(&clockwork_thread_program_v1::ID) {
                    info!("tx_sig: {} ix_program_id: {}", tx_sig, program_id);
                } else if program_id.eq(&clockwork_thread_program::ID) {
                    info!("tx_sig: {} ix_program_id: {}", tx_sig, program_id);
                }
            }

            // Instruction type
            // Transaction status
            // Compute units
            // If kickoff or exec, we want the target program
        }

        Ok(())
    }

    fn notify_block_metadata(
        &mut self,
        _blockinfo: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaBlockInfoVersions,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        false
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }
}

impl Inner {
    fn spawn<F: std::future::Future<Output = PluginResult<()>> + Send + 'static>(
        self: Arc<Self>,
        f: impl FnOnce(Arc<Self>) -> F,
    ) {
        self.runtime.spawn(f(self.clone()));
    }
}

impl Default for ClockworkMetricsPlugin {
    fn default() -> Self {
        ClockworkMetricsPlugin {
            inner: Arc::new(Inner {
                runtime: build_runtime(),
            }),
        }
    }
}

fn build_runtime() -> Arc<Runtime> {
    Arc::new(
        Builder::new_multi_thread()
            .enable_all()
            .thread_name("clockwork-plugin")
            .worker_threads(10)
            .max_blocking_threads(10)
            .build()
            .unwrap(),
    )
}
