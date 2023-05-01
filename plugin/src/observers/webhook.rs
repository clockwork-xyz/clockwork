use std::{collections::HashSet, fmt::Debug, sync::Arc};

use clockwork_webhook_program::state::Webhook;
use solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult;
use solana_program::pubkey::Pubkey;
use tokio::sync::RwLock;

pub struct WebhookObserver {
    // The set of webhook that can be processed.
    pub webhooks: RwLock<HashSet<Pubkey>>,
}

impl WebhookObserver {
    pub fn new() -> Self {
        Self {
            webhooks: RwLock::new(HashSet::new()),
        }
    }

    pub async fn observe_webhook(
        self: Arc<Self>,
        _webhook: Webhook,
        webhook_pubkey: Pubkey,
    ) -> PluginResult<()> {
        let mut w_webhooks = self.webhooks.write().await;
        w_webhooks.insert(webhook_pubkey);
        Ok(())
    }

    pub async fn process_slot(self: Arc<Self>, _slot: u64) -> PluginResult<Vec<Pubkey>> {
        let mut w_webhooks = self.webhooks.write().await;
        let executable_webhooks = w_webhooks.clone().into_iter().collect();
        w_webhooks.clear();
        Ok(executable_webhooks)
    }
}

impl Debug for WebhookObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "webhook-observer")
    }
}
