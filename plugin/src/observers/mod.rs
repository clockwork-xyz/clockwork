pub mod network;
pub mod thread;
pub mod webhook;

use std::{fmt::Debug, sync::Arc};

use network::NetworkObserver;
use thread::ThreadObserver;
use webhook::WebhookObserver;

pub struct Observers {
    pub network: Arc<NetworkObserver>,
    pub thread: Arc<ThreadObserver>,
    pub webhook: Arc<WebhookObserver>,
}

impl Debug for Observers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "observers")
    }
}
