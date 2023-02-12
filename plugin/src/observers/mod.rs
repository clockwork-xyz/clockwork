pub mod thread;
pub mod webhook;

use std::{fmt::Debug, sync::Arc};

use thread::ThreadObserver;
use webhook::WebhookObserver;

pub struct Observers {
    pub thread: Arc<ThreadObserver>,
    pub webhook: Arc<WebhookObserver>,
}

impl Observers {
    pub fn new() -> Self {
        Observers {
            thread: Arc::new(ThreadObserver::new()),
            webhook: Arc::new(WebhookObserver::new()),
        }
    }
}

impl Debug for Observers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "observers")
    }
}
