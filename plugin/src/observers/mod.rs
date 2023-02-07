pub mod automation;
pub mod webhook;

use std::{fmt::Debug, sync::Arc};

use automation::AutomationObserver;
use webhook::WebhookObserver;

pub struct Observers {
    pub automation: Arc<AutomationObserver>,
    pub webhook: Arc<WebhookObserver>,
}

impl Observers {
    pub fn new() -> Self {
        Observers {
            automation: Arc::new(AutomationObserver::new()),
            webhook: Arc::new(WebhookObserver::new()),
        }
    }
}

impl Debug for Observers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "observers")
    }
}
