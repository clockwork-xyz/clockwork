pub mod clock;
pub mod http;
pub mod pool;
pub mod queue;

use std::{fmt::Debug, sync::Arc};

use clock::ClockObserver;
use http::HttpObserver;
use pool::PoolObserver;
use queue::QueueObserver;
use solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult;

pub struct Observers {
    pub clock: Arc<ClockObserver>,
    pub http: Arc<HttpObserver>,
    pub pool: Arc<PoolObserver>,
    pub queue: Arc<QueueObserver>,
}

impl Observers {
    pub fn handle_confirmed_slot(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.clock.clone().handle_confirmed_slot(slot)?;
        self.http.clone().handle_confirmed_slot(slot)?;
        self.pool.clone().handle_confirmed_slot(slot)?;
        self.queue.clone().handle_confirmed_slot(slot)?;
        Ok(())
    }
}

impl Debug for Observers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "observers")
    }
}
