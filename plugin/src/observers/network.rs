use clockwork_client::network::objects::{Epoch, Registry};

use {
    crate::{config::PluginConfig, utils::read_or_new_keypair},
    anchor_lang::prelude::AccountMeta,
    clockwork_client::{
        network::objects::{Pool, Rotator, Snapshot, SnapshotEntry, Worker},
        Client as ClockworkClient,
    },
    log::info,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    solana_program::pubkey::Pubkey,
    solana_sdk::{signer::Signer, transaction::Transaction},
    std::{cmp::Ordering, fmt::Debug, sync::Arc},
    tokio::{runtime::Runtime, sync::RwLock},
};

static GRACE_PERIOD: u64 = 10;

pub struct NetworkObserver {
    // Plugin config values.
    pub config: PluginConfig,

    // RwLock for this node's position in the worker pools.
    pub pool_positions: Arc<RwLock<PoolPositions>>,

    // Pub worker address
    pub pubkey: Pubkey,

    // The current Clockwork registry.
    pub registry: RwLock<Registry>,

    // A copy of the current rotator account data.
    pub rotator: RwLock<Rotator>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,

    // Current snapshot of the node-stake cumulative distribution.
    // pub snapshot: RwLock<Snapshot>,

    // Sorted entries of the current snapshot.
    pub snapshot_entries: RwLock<Vec<SnapshotEntry>>,
}

impl NetworkObserver {
    pub fn new(config: PluginConfig, runtime: Arc<Runtime>) -> Self {
        Self {
            config: config.clone(),
            pool_positions: Arc::new(RwLock::new(PoolPositions::default())),
            pubkey: read_or_new_keypair(config.keypath).pubkey(),
            registry: RwLock::new(Registry {
                current_epoch_id: 0,
                locked: false,
                total_unstakes: 0,
                total_workers: 0,
            }),
            rotator: RwLock::new(Rotator {
                last_rotation_at: 0,
                nonce: 0,
            }),
            runtime,
            snapshot_entries: RwLock::new(vec![]),
        }
    }

    pub fn handle_updated_rotator(self: Arc<Self>, rotator: Rotator) -> PluginResult<()> {
        self.spawn(|this| async move {
            let mut w_rotator = this.rotator.write().await;
            *w_rotator = rotator;
            drop(w_rotator);
            Ok(())
        })
    }

    pub fn handle_updated_pool(self: Arc<Self>, pool: Pool, _slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // TODO Parse for pool name

            info!("Updated pool: {:#?}", pool);

            // Build the new pool_position
            let mut w_pool_positions = this.pool_positions.write().await;
            let workers = &mut pool.workers.clone();
            let pool_position = PoolPosition {
                current_position: pool
                    .workers
                    .iter()
                    .position(|k| k.eq(&this.pubkey))
                    .map(|i| i as u64),
                workers: workers.make_contiguous().to_vec().clone(),
            };

            // Update the pool positions struct
            match pool.name.as_str() {
                "queue" => {
                    *w_pool_positions = PoolPositions {
                        queue_pool: pool_position,
                        ..w_pool_positions.clone()
                    };
                }
                "webhook" => {
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

    // pub fn handle_updated_snapshot(self: Arc<Self>, snapshot: Snapshot) -> PluginResult<()> {
    //     self.spawn(|this| async move {
    //         if snapshot.status == SnapshotStatus::Current {
    //             let mut w_snapshot = this.snapshot.write().await;
    //             *w_snapshot = snapshot;
    //             drop(w_snapshot);
    //         }
    //         Ok(())
    //     })
    // }

    pub fn handle_updated_registry(self: Arc<Self>, registry: Registry) -> PluginResult<()> {
        self.spawn(|this| async move {
            let mut w_registry = this.registry.write().await;
            *w_registry = registry;
            drop(w_registry);
            Ok(())
        })
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            let r_rotator = this.rotator.read().await;
            info!(
                "slot: {} last_rotation: {} nonce: {}",
                confirmed_slot, r_rotator.last_rotation_at, r_rotator.nonce
            );
            drop(r_rotator);
            Ok(())
        })
    }

    /**

    TODO Come back to this

    pub async fn build_rotation_tx(
        self: Arc<Self>,
        clockwork_client: Arc<ClockworkClient>,
        slot: u64,
    ) -> PluginResult<Transaction> {
        // Acquire read locks
        let r_pool_positions = self.pool_positions.read().await;
        let r_rotator = self.rotator.read().await;
        let r_snapshot = self.snapshot.read().await;

        // Exit early if the rotator is not intialized
        if r_rotator.nonce == 0 {
            return Err(GeyserPluginError::Custom("Rotator is uninitialized".into()));
        }

        // Exit early if there is no stake in the snapshot
        if r_snapshot.total_stake == 0 {
            return Err(GeyserPluginError::Custom("No stake in snapshot".into()));
        }

        // Exit early if the pool cannot be rotated yet
        let target_slot = r_rotator.last_rotation_at + 10; // TODO Fetch the slots_per_rotation from the on-chain config account rather than using the default value
        if slot < target_slot {
            return Err(GeyserPluginError::Custom(
                "Rotator cannot be turned yet".into(),
            ));
        }

        // Exit early if this node is not in the worker pool AND
        //  we are still within the pool's grace period.
        if r_pool_positions.queue_pool.current_position.is_none()
            && slot < target_slot + GRACE_PERIOD
        {
            return Err(GeyserPluginError::Custom(
                "This node is not a worker, and it is within the rotation grace period".into(),
            ));
        }

        // Fetch the snapshot entries
        let snapshot_pubkey = Snapshot::pubkey(r_snapshot.epoch);
        let snapshot_entries = (0..r_snapshot.clone().total_workers)
            .map(|id| SnapshotEntry::pubkey(snapshot_pubkey, id))
            .map(|entry_pubkey| {
                clockwork_client
                    .get::<SnapshotEntry>(&entry_pubkey)
                    .unwrap()
            })
            .collect::<Vec<SnapshotEntry>>();

        // Build the rotation ix
        let sample = r_rotator
            .nonce
            .checked_rem(r_snapshot.total_stake)
            .unwrap_or(0);

        let entry_id = match snapshot_entries.binary_search_by(|entry| {
            if sample < entry.stake_offset {
                Ordering::Greater
            } else if sample >= entry.stake_offset
                && sample < (entry.stake_offset + entry.stake_amount)
            {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        }) {
            Err(i) => i - 1,
            Ok(i) => i,
        } as u64;
        let snapshot_pubkey =
            clockwork_client::network::objects::Snapshot::pubkey(r_snapshot.epoch);
        let entry_pubkey =
            clockwork_client::network::objects::SnapshotEntry::pubkey(snapshot_pubkey, entry_id);
        let entry = snapshot_entries.get(entry_id as usize).unwrap();
        let node = Worker::pubkey(entry_id);
        let ix = &mut clockwork_client::network::instruction::pools_rotate(
            entry_pubkey,
            node,
            clockwork_client.payer_pubkey(),
            snapshot_pubkey,
            entry.worker,
        );

        // Inject account metas for worker pools
        for pool_pubkey in r_rotator.pool_pubkeys.clone() {
            ix.accounts.push(AccountMeta::new(pool_pubkey, false));
        }

        // Drop read locks
        drop(r_pool_positions);
        drop(r_rotator);
        drop(r_snapshot);

        // Build and sign tx
        let mut tx =
            Transaction::new_with_payer(&[ix.clone()], Some(&clockwork_client.payer_pubkey()));
        tx.sign(
            &[clockwork_client.payer()],
            clockwork_client.get_latest_blockhash().map_err(|_err| {
                GeyserPluginError::Custom("Failed to get latest blockhash".into())
            })?,
        );

        Ok(tx)
    }

    */

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
    pub queue_pool: PoolPosition,
    pub webhook_pool: PoolPosition,
}

impl Default for PoolPositions {
    fn default() -> Self {
        PoolPositions {
            queue_pool: PoolPosition::default(),
            webhook_pool: PoolPosition::default(),
        }
    }
}
