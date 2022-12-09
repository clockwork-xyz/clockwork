use dashmap::DashMap;

use {
    crate::config::PluginConfig,
    clockwork_client::network::state::{Pool, Registry, Snapshot, SnapshotFrame, Worker},
    solana_geyser_plugin_interface::geyser_plugin_interface::Result as PluginResult,
    solana_program::pubkey::Pubkey,
    std::{fmt::Debug, sync::Arc},
    tokio::{runtime::Runtime, sync::RwLock},
};

pub struct NetworkObserver {
    // Plugin config values.
    pub config: PluginConfig,

    // RwLock for this node's position in the worker pools.
    pub pool_positions: Arc<RwLock<PoolPositions>>,

    // A cache of the network registry.
    pub registry: Arc<RwLock<Registry>>,

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
            pool_positions: Arc::new(RwLock::new(PoolPositions::default())),
            registry: Arc::new(RwLock::new(Registry {
                current_epoch: 0,
                locked: false,
                nonce: 0,
                total_pools: 0,
                total_unstakes: 0,
                total_workers: 0,
            })),
            snapshots: DashMap::new(),
            snapshot_frames: DashMap::new(),
            runtime,
        }
    }

    pub fn observe_pool(self: Arc<Self>, pool: Pool, _slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Build the new pool_position
            let worker_pubkey = Worker::pubkey(this.config.worker_id);
            let mut w_pool_positions = this.pool_positions.write().await;
            let workers = &mut pool.workers.clone();
            let pool_position = PoolPosition {
                current_position: pool
                    .workers
                    .iter()
                    .position(|k| k.eq(&worker_pubkey))
                    .map(|i| i as u64),
                workers: workers.make_contiguous().to_vec().clone(),
            };

            // Update the pool positions struct
            match pool.id {
                0 => {
                    *w_pool_positions = PoolPositions {
                        thread_pool: pool_position,
                        ..w_pool_positions.clone()
                    };
                }
                1 => {
                    *w_pool_positions = PoolPositions {
                        webhook_pool: pool_position,
                        ..w_pool_positions.clone()
                    };
                }
                _ => {}
            }

            drop(w_pool_positions);
            Ok(())
        })
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
            // Update the registry.
            let mut w_registry = this.registry.write().await;
            *w_registry = registry.clone();
            drop(w_registry);

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

#[derive(Clone)]
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

#[derive(Clone)]
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
