use std::{fmt::Debug, sync::Arc};

use anchor_lang::prelude::Pubkey;
use clockwork_webhook_program::state::Webhook;
use clockwork_relayer_api::Relay;
use log::info;
use reqwest::header::CONTENT_TYPE;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult;

use crate::config::PluginConfig;

use super::AccountGet;

pub struct WebhookExecutor {
    pub config: PluginConfig,
}

impl WebhookExecutor {
    pub fn new(config: PluginConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn execute_webhooks(
        self: Arc<Self>,
        client: Arc<RpcClient>,
        pubkeys: Vec<Pubkey>,
    ) -> PluginResult<()> {
        // self.spawn(|this| async move {
        //     let url = http_request.clone().request.url;
        //     let res = match http_request.request.method {
        //         HttpMethod::Get => this.client.get(url),
        //         HttpMethod::Post => this.client.post(url),
        //     }
        //     .header("x-caller-id", http_request.request.caller.to_string())
        //     .header("x-request-id", http_request.pubkey.to_string())
        //     .header("x-worker-id", this.worker_id.to_string())
        //     .send()
        //     .await;
        //     match res {
        //         Ok(res) => info!("Webhook response: {:#?}", res),
        //         Err(err) => info!("Webhook request failed with error: {}", err),
        //     }
        //     this.observers
        //         .webhook
        //         .webhook_requests
        //         .remove(&http_request);
        //     Ok(())
        // })
        // TODO Route to correct relayer
        for webhook_pubkey in pubkeys {
            let webhook = client
                .clone()
                .get::<Webhook>(&webhook_pubkey)
                .await
                .unwrap();
            info!("webhook: {} {:?}", webhook_pubkey, webhook);
            let url = "http://127.0.0.1:8000/relay";
            let client = reqwest::Client::new();
            // for request_pubkey in requests {
            let _res = dbg!(
                client
                    .post(url)
                    .header(CONTENT_TYPE, "application/json")
                    .json(&Relay {
                        webhook: webhook_pubkey,
                    })
                    .send()
                    .await
            );
        }
        // }
        Ok(())
    }
}

impl Debug for WebhookExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "webhook-executor")
    }
}
