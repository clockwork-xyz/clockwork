pub mod tx;
pub mod webhook;

use std::{fmt::Debug, sync::Arc};

use solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult;
use tokio::runtime::Runtime;
use tx::TxExecutor;
use webhook::WebhookExecutor;

pub struct Executors {
    pub tx: Arc<TxExecutor>,
    pub runtime: Arc<Runtime>,
    pub webhook: Arc<WebhookExecutor>,
}

impl Executors {
    pub fn execute_work(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.tx.clone().execute_txs(slot)?;
            this.webhook.clone().execute_requests()?;
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

impl Debug for Executors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "executors")
    }
}
