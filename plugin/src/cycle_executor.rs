use cronos_client::{
    network::state::{Cycler, Snapshot, SnapshotStatus},
    pool::state::Pool,
    Client as CronosClient,
};
use crossbeam::channel::Sender;
use log::info;

use {
    crate::{config::PluginConfig, tpu_client::TpuClient},
    dashmap::DashMap,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{
        commitment_config::CommitmentConfig, signature::Signature, transaction::Transaction,
    },
    std::{
        fmt::Debug,
        sync::{Arc, RwLock},
    },
    tokio::runtime::Runtime,
};

static LOCAL_RPC_URL: &str = "http://127.0.0.1:8899";
static LOCAL_WEBSOCKET_URL: &str = "ws://127.0.0.1:8900";

pub struct DelegateStatus {
    pub executor_pool_position: Option<u64>,
}

pub struct CycleExecutor {
    // Plugin config values.
    pub config: PluginConfig,

    pub cycler: RwLock<Cycler>,

    // The currently active executor delegates.
    pub executor_delegates: DashMap<usize, Pubkey>,

    // Map from slot numbers to delegate pools.
    pub pools: DashMap<u64, Pool>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,

    pub snapshot: RwLock<Snapshot>,

    //
    pub status_sender: Sender<DelegateStatus>,

    // Map from tx signatures to slot when the tx was sent.
    pub tx_signatures: DashMap<Signature, u64>,
}

impl CycleExecutor {
    pub fn new(
        config: PluginConfig,
        runtime: Arc<Runtime>,
        status_sender: Sender<DelegateStatus>,
    ) -> Self {
        Self {
            config: config.clone(),
            cycler: RwLock::new(Cycler {
                last_cycle_at: 0,
                nonce: 0,
            }),
            executor_delegates: DashMap::new(),
            pools: DashMap::new(),
            runtime,
            snapshot: RwLock::new(Snapshot {
                entry_count: 0,
                id: 0,
                stake_total: 0,
                status: SnapshotStatus::Current,
            }),
            status_sender,
            tx_signatures: DashMap::new(),
        }
    }

    pub fn handle_updated_cycler(self: Arc<Self>, cycler: Cycler) -> PluginResult<()> {
        self.spawn(|this| async move {
            let mut w = this.cycler.write().unwrap();
            *w = cycler;
            Ok(())
        })
    }

    pub fn handle_updated_pool(self: Arc<Self>, pool: Pool, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.pools.insert(slot, pool);
            Ok(())
        })
    }

    pub fn handle_updated_snapshot(self: Arc<Self>, snapshot: Snapshot) -> PluginResult<()> {
        self.spawn(|this| async move {
            if snapshot.status == SnapshotStatus::Current {
                let mut w = this.snapshot.write().unwrap();
                *w = snapshot;
            }
            Ok(())
        })
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            let r = this.cycler.read().unwrap();
            info!(
                "confirmed_slot: {} last_cycle_at: {} nonce: {}",
                confirmed_slot, r.last_cycle_at, r.nonce
            );

            // Update the set of executor delegates.
            this.pools.retain(|slot, pool| {
                if *slot == confirmed_slot {
                    this.executor_delegates.clear();
                    pool.delegates
                        .make_contiguous()
                        .iter()
                        .enumerate()
                        .for_each(|(i, pubkey)| {
                            this.executor_delegates.insert(i, *pubkey);
                        });
                }
                *slot > confirmed_slot
            });

            // Cycle the pool
            let r_cycler = this.cycler.read().unwrap();
            if confirmed_slot >= r_cycler.last_cycle_at + 10 {
                // TODO read slots_per_cycle from config
                this.clone().cycle_pool()?;
            }

            Ok(())
        })
    }

    fn cycle_pool(self: Arc<Self>) -> PluginResult<()> {
        self.spawn(|this| async move {
            let r_cycler = this.cycler.read().unwrap();
            let r_snapshot = this.snapshot.read().unwrap();

            // Exit early if cycle will fail
            if r_cycler.nonce == 0 || r_snapshot.stake_total == 0 {
                return Ok(());
            }

            info!("Attempting to cycle the pool");

            // Create a new tpu client
            let tpu_client = TpuClient::new(
                this.config.delegate_keypath.clone(),
                LOCAL_RPC_URL.into(),
                LOCAL_WEBSOCKET_URL.into(),
            );

            // Create a cronos client
            let cronos_client =
                CronosClient::new(this.config.delegate_keypath.clone(), LOCAL_RPC_URL.into());

            // Build cycle ix

            let snapshot_pubkey = cronos_client::network::state::Snapshot::pda(r_snapshot.id).0;
            let entry_pubkey =
                cronos_client::network::state::SnapshotEntry::pda(snapshot_pubkey, 0).0; // TODO Get correct entry
            let ix = cronos_client::network::instruction::cycler_run(
                entry_pubkey,
                cronos_client.payer_pubkey(),
                snapshot_pubkey,
            );

            // Submit tx
            let mut tx = Transaction::new_with_payer(&[ix], Some(&cronos_client.payer_pubkey()));
            tx.sign(
                &[cronos_client.payer()],
                cronos_client.get_latest_blockhash().map_err(|_err| {
                    GeyserPluginError::Custom("Failed to get latest blockhash".into())
                })?,
            );

            info!("Tx: {}", tx.signatures[0]);
            tpu_client.send_transaction(&tx);

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

impl Debug for CycleExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cycler")
    }
}
