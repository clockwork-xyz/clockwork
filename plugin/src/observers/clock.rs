use {
    dashmap::DashMap,
    solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult,
    solana_program::clock::Clock,
    std::{fmt::Debug, sync::Arc},
    tokio::runtime::Runtime,
};

pub struct ClockObserver {
    //
    pub confirmed_clock: Option<Clock>,

    // Map from slot numbers to the sysvar clock data for that slot.
    pub unconfirmed_clocks: DashMap<u64, Clock>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl ClockObserver {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            confirmed_clock: None,
            unconfirmed_clocks: DashMap::new(),
            runtime,
        }
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.unconfirmed_clocks.retain(|slot, clock| {
                // TODO Update the confirmed clock
                *slot > confirmed_slot
            });
            Ok(())
        })
    }

    pub fn handle_updated_clock(self: Arc<Self>, clock: Clock) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.unconfirmed_clocks.insert(clock.slot, clock);
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

impl Debug for ClockObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "clock-observer")
    }
}
