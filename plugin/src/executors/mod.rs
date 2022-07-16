pub mod http;
pub mod tx;

use std::{fmt::Debug, sync::Arc};

use http::HttpExecutor;
use solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult;
use tokio::runtime::Runtime;
use tx::TxExecutor;

use crate::observers::Observers;

pub struct Executors {
    pub http: Arc<HttpExecutor>,
    pub tx: Arc<TxExecutor>,
    pub observers: Arc<Observers>,
    pub runtime: Arc<Runtime>,
}

impl Executors {
    pub fn handle_confirmed_slot(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Update observers
            this.observers.clone().handle_confirmed_slot(slot)?;

            // Execute work
            this.tx.clone().execute_txs(slot)?;
            this.http.clone().execute_requests()?;
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
