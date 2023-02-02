use std::{
    collections::HashSet,
    fmt::Debug,
    hash::{Hash, Hasher},
    sync::Arc,
};

use clockwork_client::webhook::state::Request;
use solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult;
use solana_program::pubkey::Pubkey;
use tokio::sync::RwLock;

pub struct WebhookObserver {
    // The set of http request pubkeys that can be processed.
    pub webhook_requests: RwLock<HashSet<HttpRequest>>,
}

impl WebhookObserver {
    pub fn new() -> Self {
        Self {
            webhook_requests: RwLock::new(HashSet::new()),
        }
    }

    pub async fn observe_request(self: Arc<Self>, request: HttpRequest) -> PluginResult<()> {
        let mut w_webhook_requests = self.webhook_requests.write().await;
        w_webhook_requests.insert(request);
        drop(w_webhook_requests);
        Ok(())
    }
}

impl Debug for WebhookObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http-observer")
    }
}

/**
 * HttpRequest
 */

#[derive(Clone)]
pub struct HttpRequest {
    pub pubkey: Pubkey,
    pub request: Request,
}

impl Hash for HttpRequest {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pubkey.hash(state);
    }
}

impl PartialEq for HttpRequest {
    fn eq(&self, other: &Self) -> bool {
        self.pubkey == other.pubkey
    }
}

impl Eq for HttpRequest {}
