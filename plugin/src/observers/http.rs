use {
    clockwork_client::http::state::Request,
    dashmap::{DashMap, DashSet},
    log::info,
    solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult,
    solana_program::pubkey::Pubkey,
    std::{
        fmt::Debug,
        hash::{Hash, Hasher},
        sync::Arc,
    },
    tokio::runtime::Runtime,
};

pub struct HttpObserver {
    // The set of http request pubkeys that can be processed.
    pub confirmed_requests: DashSet<HttpRequest>,

    // Map from slot numbers to the list of http requests that arrived at that slot.
    pub unconfirmed_requests: DashMap<u64, DashSet<HttpRequest>>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl HttpObserver {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            confirmed_requests: DashSet::new(),
            unconfirmed_requests: DashMap::new(),
            runtime,
        }
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.unconfirmed_requests.retain(|slot, request_pubkeys| {
                if *slot <= confirmed_slot {
                    request_pubkeys.iter().for_each(|request| {
                        this.confirmed_requests.insert(request.clone());
                    });
                    false
                } else {
                    true
                }
            });

            Ok(())
        })
    }

    pub fn handle_updated_http_request(
        self: Arc<Self>,
        request: HttpRequest,
        slot: u64,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            info!("Caching http request: {:#?}", request.pubkey);
            this.confirmed_requests.remove(&request);
            this.unconfirmed_requests
                .entry(slot)
                .and_modify(|v| {
                    v.insert(request.clone());
                })
                .or_insert_with(|| {
                    let v = DashSet::new();
                    v.insert(request);
                    v
                });
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

impl Debug for HttpObserver {
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
