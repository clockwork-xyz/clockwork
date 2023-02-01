pub mod tx;
pub mod webhook;

use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use clockwork_client::Client as ClockworkClient;
use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult;
use tokio::runtime::Runtime;
use tx::TxExecutor;
use webhook::WebhookExecutor;

use crate::{observers::Observers, tpu_client::TpuClient};

pub struct Executors {
    pub tx: Arc<TxExecutor>,
    pub webhook: Arc<WebhookExecutor>,
    pub client: Arc<ClockworkClient>,
    pub lock: AtomicBool,
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
        if self
            .clone()
            .lock
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            info!("processed_slot: {} duration: {:?}", slot, now.elapsed());
            return Ok(());
        }

        // Process the slot on the observers.
        let executable_threads = observers.thread.clone().process_slot(slot)?;

        // Process the slot in the transaction executor.
        self.tx
            .clone()
            .execute_txs(
                self.client.clone(),
                executable_threads,
                slot,
                runtime.clone(),
                tpu_client,
            )
            .await?;

        // Release the lock.
        self.clone()
            .lock
            .store(false, std::sync::atomic::Ordering::Relaxed);
        info!("processed_slot: {} duration: {:?}", slot, now.elapsed());
        Ok(())
    }
}

impl Debug for Executors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "executors")
    }
}
