use std::{fmt::Debug, sync::Arc};

use cronos_client::Client as CronosClient;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, Result as PluginResult,
};
use solana_sdk::{signature::Signature, transaction::Transaction};
use tokio::runtime::Runtime;

use crate::{
    config::PluginConfig, delegate::Delegate, scheduler::Scheduler, tpu_client::TpuClient,
};

pub struct Executor {
    pub config: PluginConfig,
    pub cronos_client: Arc<CronosClient>,
    pub delegate: Arc<Delegate>,
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
            runtime,
            scheduler,
            tpu_client,
        }
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Rotate the pools
            this.delegate.clone().handle_confirmed_slot(slot)?;
            this.delegate
                .clone()
                .build_rotation_tx(this.cronos_client.clone(), slot)
                .await
                .and_then(|tx| this.clone().simulate_tx(&tx))
                .and_then(|tx| this.clone().submit_tx(&tx))
                .and_then(|sig| this.clone().cache_signature(slot, sig))
                .ok();

            // Execute scheduled tasks
            this.scheduler.clone().handle_confirmed_slot(slot)?;
            this.scheduler
                .clone()
                .build_queue_txs(this.cronos_client.clone())
                .await
                .iter()
                .filter_map(|tx| this.clone().simulate_tx(tx).map_or(None, |tx| Some(tx)))
                .filter_map(|tx| this.clone().submit_tx(&tx).map_or(None, |sig| Some(sig)))
                .for_each(|sig| {
                    this.clone().cache_signature(slot, sig).ok();
                });

            Ok(())
        })
    }

    fn simulate_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<Transaction> {
        self.tpu_client
            .rpc_client()
            .simulate_transaction(tx)
            .map_err(|_| GeyserPluginError::Custom("Tx failed simulation".into()))
            .map(|response| {
                if response.value.err.is_some() {
                    Err(GeyserPluginError::Custom("Tx failed simulation".into()))
                } else {
                    Ok(tx.clone())
                }
            })?
    }

    fn submit_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<Signature> {
        let sig = tx.signatures[0];

        // TODO dedupe this tx signature

        if !self.tpu_client.send_transaction(tx) {
            return Err(GeyserPluginError::Custom(
                "Failed to send transaction".into(),
            ));
        }

        Ok(sig)
    }

    fn cache_signature(self: Arc<Self>, _slot: u64, _sig: Signature) -> PluginResult<()> {
        // TODO Index this signature against it's slot

        // TODO Check on all the signatures in the last X slots

        // TODO For txs that haven't been confirmed in X slots, consider them as "timed out" and retry with a linear backoff

        // TODO Metrics to track the success of every tx

        Ok(())
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
