pub mod network;
pub mod queue;
pub mod webhook;

use {
    solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult,
    std::{fmt::Debug, sync::Arc},
};

use network::NetworkObserver;
use queue::QueueObserver;
use webhook::WebhookObserver;

pub struct Observers {
    pub network: Arc<NetworkObserver>,
    pub queue: Arc<QueueObserver>,
    pub webhook: Arc<WebhookObserver>,
}

impl Observers {
    pub fn handle_confirmed_slot(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.network.clone().handle_confirmed_slot(slot)?;
        self.queue.clone().handle_confirmed_slot(slot)?;
        self.webhook.clone().handle_confirmed_slot(slot)?;
        Ok(())
    }
}

impl Debug for Observers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "observers")
    }
}
