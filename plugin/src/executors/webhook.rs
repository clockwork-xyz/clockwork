use {
    crate::{
        config::PluginConfig,
        observers::{webhook::HttpRequest, Observers},
    },
    clockwork_client::webhook::objects::HttpMethod,
    log::info,
    solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult,
    solana_program::pubkey::Pubkey,
    std::{fmt::Debug, sync::Arc},
    tokio::runtime::Runtime,
};

pub struct WebhookExecutor {
    pub config: PluginConfig,
    pub client: reqwest::Client,
    pub observers: Arc<Observers>,
    pub runtime: Arc<Runtime>,
    pub worker_id: Pubkey,
}

impl WebhookExecutor {
    pub fn new(
        config: PluginConfig,
        observers: Arc<Observers>,
        runtime: Arc<Runtime>,
        worker_id: Pubkey,
    ) -> Self {
        Self {
            config: config.clone(),
            client: reqwest::Client::new(),
            observers,
            runtime,
            worker_id,
        }
    }

    pub fn execute_requests(self: Arc<Self>) -> PluginResult<()> {
        for request in self.clone().observers.webhook.webhook_requests.iter() {
            self.clone().execute_request(request.clone())?;
        }
        Ok(())
    }

    fn execute_request(self: Arc<Self>, http_request: HttpRequest) -> PluginResult<()> {
        self.spawn(|this| async move {
            let url = http_request.clone().request.url;
            let res = match http_request.request.method {
                HttpMethod::Get => this.client.get(url),
                HttpMethod::Post => this.client.post(url),
            }
            .header("x-caller-id", http_request.request.caller.to_string())
            .header("x-request-id", http_request.pubkey.to_string())
            .header("x-worker-id", this.worker_id.to_string())
            .send()
            .await;
            match res {
                Ok(res) => info!("Webhook response: {:#?}", res),
                Err(err) => info!("Webhook request failed with error: {}", err),
            }
            this.observers
                .webhook
                .webhook_requests
                .remove(&http_request);
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

impl Debug for WebhookExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http-executor")
    }
}
