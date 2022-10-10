pub mod network;
pub mod queue;
pub mod webhook;

use std::{fmt::Debug, sync::Arc};

use network::NetworkObserver;
use queue::QueueObserver;
use webhook::WebhookObserver;

pub struct Observers {
    pub network: Arc<NetworkObserver>,
    pub queue: Arc<QueueObserver>,
    pub webhook: Arc<WebhookObserver>,
}

impl Debug for Observers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "observers")
    }
}
