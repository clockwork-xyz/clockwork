use {
    clockwork_client::webhook::state::Request,
    dashmap::DashSet,
    solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult,
    solana_program::pubkey::Pubkey,
    std::{
        fmt::Debug,
        hash::{Hash, Hasher},
        sync::Arc,
    },
    tokio::runtime::Runtime,
};

pub struct WebhookObserver {
    // The set of http request pubkeys that can be processed.
    pub webhook_requests: DashSet<HttpRequest>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl WebhookObserver {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            webhook_requests: DashSet::new(),
            runtime,
        }
    }

    pub fn observe_request(self: Arc<Self>, request: HttpRequest) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.webhook_requests.insert(request);
            Ok(())
        })
    }

    // fn build_reqwests() ->

    fn spawn<F: std::future::Future<Output = PluginResult<()>> + Send + 'static>(
        self: &Arc<Self>,
        f: impl FnOnce(Arc<Self>) -> F,
    ) -> PluginResult<()> {
        self.runtime.spawn(f(self.clone()));
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
