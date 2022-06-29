use std::{fmt::Debug, sync::Arc};

use cronos_client::Client as CronosClient;
use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult;
use solana_program::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use tokio::runtime::Runtime;

use crate::{
    config::PluginConfig, delegate::Delegate, scheduler::Scheduler, tpu_client::TpuClient,
    utils::read_or_new_keypair,
};

pub struct Executor {
    pub config: PluginConfig,
    pub cronos_client: Arc<CronosClient>,
    pub delegate: Arc<Delegate>,
    pub delegate_pubkey: Pubkey,
    pub runtime: Arc<Runtime>,
    pub scheduler: Arc<Scheduler>,
    pub tpu_client: Arc<TpuClient>,
}

impl Executor {
    pub fn new(
        config: PluginConfig,
        cronos_client: Arc<CronosClient>,
        delegate: Arc<Delegate>,
        runtime: Arc<Runtime>,
        scheduler: Arc<Scheduler>,
        tpu_client: Arc<TpuClient>,
    ) -> Self {
        Self {
            config: config.clone(),
            cronos_client,
            delegate,
            delegate_pubkey: read_or_new_keypair(config.delegate_keypath).pubkey(),
            runtime,
            scheduler,
            tpu_client,
        }
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Send transactions given the current state
            info!("slot: {}", confirmed_slot);

            this.delegate.clone().try_rotate_pool(
                this.cronos_client.clone(),
                confirmed_slot,
                this.tpu_client.clone(),
            )?;

            Ok(())
        })
    }

    fn spawn<F: std::future::Future<Output = PluginResult<()>> + Send + 'static>(
        self: &Arc<Self>,
        f: impl FnOnce(Arc<Self>) -> F,
    ) -> PluginResult<()> {
        self.runtime.spawn(f(self.clone()));
        Ok(())
    }
}

impl Debug for Executor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Executor")
    }
}
