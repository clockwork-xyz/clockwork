use std::{collections::HashSet, fmt::Debug, sync::Arc};

use clockwork_client::webhook::state::Request;
use solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult;
use solana_program::pubkey::Pubkey;
use tokio::sync::RwLock;

pub struct WebhookObserver {
    // The set of webhook requests that can be processed.
    pub webhook_requests: RwLock<HashSet<Pubkey>>,
}

impl WebhookObserver {
    pub fn new() -> Self {
        Self {
            webhook_requests: RwLock::new(HashSet::new()),
        }
    }

    pub async fn observe_request(
        self: Arc<Self>,
        _request: Request,
        request_pubkey: Pubkey,
        _slot: u64,
    ) -> PluginResult<()> {
        let mut w_webhook_requests = self.webhook_requests.write().await;
        w_webhook_requests.insert(request_pubkey);
        drop(w_webhook_requests);
        Ok(())
    }
}

impl Debug for WebhookObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "webhook-observer")
    }
}
