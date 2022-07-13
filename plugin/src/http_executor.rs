use log::info;

use {
    crate::config::PluginConfig,
    cronos_client::http::state::{HttpMethod, Request},
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    std::{fmt::Debug, sync::Arc},
    tokio::runtime::Runtime,
};

pub struct HttpExecutor {
    pub config: PluginConfig,
    pub runtime: Arc<Runtime>,
}

impl HttpExecutor {
    pub fn new(config: PluginConfig, runtime: Arc<Runtime>) -> Self {
        Self {
            config: config.clone(),
            runtime,
        }
    }

    pub fn handle_updated_request(self: Arc<Self>, request: Request) -> PluginResult<()> {
        self.spawn(|_this| async move {
            let client = reqwest::Client::new();
            let response = match request.method {
                HttpMethod::Get => client.get(request.clone().url),
                HttpMethod::Post => client.post(request.clone().url),
            }
            .send()
            .await
            .map_err(|err| GeyserPluginError::Custom(err.into()))?;
            info!("Http response: {:#?}", response);
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

impl Debug for HttpExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Executor")
    }
}
