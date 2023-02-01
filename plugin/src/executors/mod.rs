pub mod tx;
pub mod webhook;

use std::{fmt::Debug, sync::Arc};

use clockwork_client::Client as ClockworkClient;
use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult;
use tokio::runtime::Runtime;
use tx::TxExecutor;
use webhook::WebhookExecutor;

use crate::{config::PluginConfig, observers::Observers, tpu_client::TpuClient};

pub struct Executors {
    pub tx: Arc<TxExecutor>,
    pub webhook: Arc<WebhookExecutor>,
    pub client: Arc<ClockworkClient>,
}

impl Executors {
    pub async fn process_slot(
        self: Arc<Self>,
        observers: Arc<Observers>,
        slot: u64,
        runtime: Arc<Runtime>,
        tpu_client: Arc<TpuClient>,
    ) -> PluginResult<()> {
        info!("process_slot: {}", slot,);
        let now = std::time::Instant::now();

        // Process the slot on the observers.
        observers.thread.clone().observe_processed_slot(slot)?;

        // Process the slot in the transaction executor.
        self.tx.clone().execute_txs(
            observers.clone(),
            self.client.clone(),
            slot,
            runtime.clone(),
            tpu_client,
        )?;

        info!("processed_slot: {} duration: {:?}", slot, now.elapsed());

        // Process the slot in the webhook executor.
        // self.webhook.clone().execute_requests()?;
        Ok(())
    }
}

impl Debug for Executors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "executors")
    }
}
