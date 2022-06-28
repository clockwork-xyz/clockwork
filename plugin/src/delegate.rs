use {
    crate::{config::PluginConfig, tpu_client::TpuClient, utils::read_or_new_keypair},
    cronos_client::{
        network::state::{Rotator, Snapshot, SnapshotEntry, SnapshotStatus},
        pool::state::Pool,
        Client as CronosClient,
    },
    dashmap::DashMap,
    log::info,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    solana_program::pubkey::Pubkey,
    solana_sdk::{signature::Signature, signer::Signer, transaction::Transaction},
    std::{cmp::Ordering, fmt::Debug, sync::Arc},
    tokio::{runtime::Runtime, sync::RwLock},
};

static LOCAL_RPC_URL: &str = "http://127.0.0.1:8899";
static LOCAL_WEBSOCKET_URL: &str = "ws://127.0.0.1:8900";

pub struct Delegate {
    // Plugin config values.
    pub config: PluginConfig,

    // Pub delegate keypiar
    pub delegate_pubkey: Pubkey,

    // Map from unconfirmed slot numbers to the expected delegate pool for that moment.
    pub pool_forecasts: DashMap<u64, Pool>,

    // RwLock for this node's delegate status.
    pub pool_positions: Arc<RwLock<PoolPositions>>,

    // A copy of the current rotator account data.
    pub rotator: RwLock<Rotator>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,

    // Current snapshot of the node-stake cumulative distribution.
    pub snapshot: RwLock<Snapshot>,

    // Sorted entries of the snapshot.
    pub snapshot_entries: RwLock<Vec<SnapshotEntry>>,

    // Map from target slot numbers to signatures for rotation txs.
    pub tx_signatures: DashMap<u64, Signature>,
}

impl Delegate {
    pub fn new(config: PluginConfig, runtime: Arc<Runtime>) -> Self {
        Self {
            config: config.clone(),
            delegate_pubkey: read_or_new_keypair(config.delegate_keypath).pubkey(),
            pool_forecasts: DashMap::new(),
            pool_positions: Arc::new(RwLock::new(PoolPositions::default())),
            rotator: RwLock::new(Rotator {
                last_slot: 0,
                nonce: 0,
            }),
            runtime,
            snapshot: RwLock::new(Snapshot {
                id: 0,
                node_count: 0,
                stake_total: 0,
                status: SnapshotStatus::Current,
            }),
            snapshot_entries: RwLock::new(vec![]),
            tx_signatures: DashMap::new(),
        }
    }

    pub fn handle_updated_rotator(self: Arc<Self>, rotator: Rotator) -> PluginResult<()> {
        self.spawn(|this| async move {
            let mut w = this.rotator.write().await;
            *w = rotator;
            Ok(())
        })
    }

    pub fn handle_updated_pool(self: Arc<Self>, pool: Pool, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.pool_forecasts.insert(slot, pool);
            Ok(())
        })
    }

    pub fn handle_updated_snapshot(self: Arc<Self>, snapshot: Snapshot) -> PluginResult<()> {
        self.spawn(|this| async move {
            if snapshot.status == SnapshotStatus::Current {
                let mut w_snapshot = this.snapshot.write().await;
                *w_snapshot = snapshot;
                drop(w_snapshot);
            }
            Ok(())
        })
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Aquire read locks
            let r_rotator = this.rotator.read().await;
            info!(
                "slot: {} last_rotation: {} nonce: {}",
                confirmed_slot, r_rotator.last_slot, r_rotator.nonce
            );

            // Update the set delegate status
            let mut w_pool_positions = this.pool_positions.write().await;
            this.pool_forecasts.retain(|slot, pool| {
                if *slot == confirmed_slot {
                    *w_pool_positions = PoolPositions {
                        scheduler_pool_position: PoolPosition {
                            current_position: pool
                                .delegates
                                .iter()
                                .position(|k| k.eq(&this.delegate_pubkey))
                                .map(|i| i as u64),
                            delegates: pool.delegates.make_contiguous().to_vec().clone(),
                        },
                    }
                }
                *slot > confirmed_slot
            });
            drop(w_pool_positions);

            // Rotate the pool
            this.clone().rotate_pool(confirmed_slot)?;

            // Drop read locks
            drop(r_rotator);

            Ok(())
        })
    }

    fn rotate_pool(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Acquire read locks
            let r_pool_positions = this.pool_positions.read().await;
            let r_rotator = this.rotator.read().await;
            let r_snapshot = this.snapshot.read().await;

            // Exit early this this node is not in the scheduler
            // TODO Should rotations fall to another pool?
            if r_pool_positions
                .scheduler_pool_position
                .current_position
                .is_none()
            {
                return Ok(());
            }

            // Exit early if it's not time to rotate the pool
            let target_slot = r_rotator.last_slot + 10; // TODO Fetch the slots_per_rotation from the on-chain config account rather than using the default value
            if slot < target_slot {
                return Ok(());
            }

            // Fetch the snapshot entries
            let cronos_client = CronosClient::new(
                read_or_new_keypair(this.config.clone().delegate_keypath),
                LOCAL_RPC_URL.into(),
            );
            let snapshot_pubkey = Snapshot::pda(r_snapshot.id).0;
            let snapshot_entries = (0..r_snapshot.clone().node_count)
                .map(|id| SnapshotEntry::pda(snapshot_pubkey, id).0)
                .map(|entry_pubkey| cronos_client.get::<SnapshotEntry>(&entry_pubkey).unwrap())
                .collect::<Vec<SnapshotEntry>>();

            // Exit early if cycle will fail
            if r_rotator.nonce == 0 || r_snapshot.stake_total == 0 {
                return Ok(());
            }

            // Build cycle ix
            let sample = r_rotator
                .nonce
                .checked_rem(r_snapshot.stake_total)
                .unwrap_or(0);
            let entry_id = snapshot_entries
                .binary_search_by(|entry| {
                    if sample < entry.stake_offset {
                        Ordering::Less
                    } else if sample >= entry.stake_offset
                        && sample < (entry.stake_offset + entry.stake_amount)
                    {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                })
                .unwrap() as u64;

            // Build the pool rotation ix
            let snapshot_pubkey = cronos_client::network::state::Snapshot::pda(r_snapshot.id).0;
            let entry_pubkey =
                cronos_client::network::state::SnapshotEntry::pda(snapshot_pubkey, entry_id).0; // TODO Get correct entry
            let ix = cronos_client::network::instruction::rotator_turn(
                entry_pubkey,
                cronos_client.payer_pubkey(),
                snapshot_pubkey,
            );

            // Sign tx
            let mut tx = Transaction::new_with_payer(&[ix], Some(&cronos_client.payer_pubkey()));
            tx.sign(
                &[cronos_client.payer()],
                cronos_client.get_latest_blockhash().map_err(|_err| {
                    GeyserPluginError::Custom("Failed to get latest blockhash".into())
                })?,
            );

            // Exit early if this node has already submitted a rotation tx for this target slot.
            let sig = tx.signatures[0];
            if this.tx_signatures.contains_key(&target_slot) {
                return Ok(());
            }
            this.tx_signatures.insert(target_slot, sig);

            // Submit tx
            TpuClient::new(
                read_or_new_keypair(this.config.clone().delegate_keypath),
                LOCAL_RPC_URL.into(),
                LOCAL_WEBSOCKET_URL.into(),
            )
            .send_transaction(&tx);
            info!("Pool rotation: {}", sig);

            // TODO Confirm sigs and retry logic

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

impl Debug for Delegate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Delegate")
    }
}

pub struct PoolPosition {
    pub current_position: Option<u64>,
    pub delegates: Vec<Pubkey>,
}

pub struct PoolPositions {
    pub scheduler_pool_position: PoolPosition,
}

impl Default for PoolPositions {
    fn default() -> Self {
        PoolPositions {
            scheduler_pool_position: PoolPosition {
                current_position: None,
                delegates: vec![],
            },
        }
    }
}
