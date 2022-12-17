use {
    crate::config::PluginConfig,
    clockwork_client::network::state::{Registry, Snapshot, SnapshotFrame},
    dashmap::DashMap,
    solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult,
    solana_program::pubkey::Pubkey,
    std::{fmt::Debug, sync::Arc},
    tokio::runtime::Runtime,
};

pub struct NetworkObserver {
    // Plugin config values.
    pub config: PluginConfig,

    // A cache of the network's snapshot accounts, indexed by epoch.
    pub snapshots: DashMap<u64, Snapshot>,

    // A cache of the worker's snapshot frames, indexed by snapshot address.
    pub snapshot_frames: DashMap<Pubkey, SnapshotFrame>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl NetworkObserver {
    pub fn new(config: PluginConfig, runtime: Arc<Runtime>) -> Self {
        Self {
            config: config.clone(),
            // pool_positions: Arc::new(RwLock::new(PoolPositions::default())),
            snapshots: DashMap::new(),
            snapshot_frames: DashMap::new(),
            runtime,
        }
    }

    pub fn observe_snapshot(self: Arc<Self>, snapshot: Snapshot) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.snapshots.insert(snapshot.id, snapshot);
            Ok(())
        })
    }

    pub fn observe_snapshot_frame(
        self: Arc<Self>,
        snapshot_frame: SnapshotFrame,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            if snapshot_frame.id.eq(&this.config.worker_id) {
                this.snapshot_frames
                    .insert(snapshot_frame.snapshot, snapshot_frame);
            }
            Ok(())
        })
    }

    pub fn observe_registry(self: Arc<Self>, registry: Registry) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Drop old snapshots and snapshot frames.
            this.snapshots.retain(|id, _| {
                if registry.current_epoch > *id {
                    let snapshot_pubkey = Snapshot::pubkey(*id);
                    this.snapshot_frames.remove(&snapshot_pubkey);
                    false
                } else {
                    true
                }
            });

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

impl Debug for NetworkObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "network-observer")
    }
}

#[derive(Clone, Debug)]
pub struct PoolPosition {
    pub current_position: Option<u64>,
    pub workers: Vec<Pubkey>,
}

impl Default for PoolPosition {
    fn default() -> Self {
        PoolPosition {
            current_position: None,
            workers: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct PoolPositions {
    pub thread_pool: PoolPosition,
    pub webhook_pool: PoolPosition,
}

impl Default for PoolPositions {
    fn default() -> Self {
        PoolPositions {
            thread_pool: PoolPosition::default(),
            webhook_pool: PoolPosition::default(),
        }
    }
}
