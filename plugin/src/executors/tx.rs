use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use async_once::AsyncOnce;
use clockwork_client::{
    network::state::{Pool, Registry, Snapshot, SnapshotFrame, Worker},
    thread::state::Thread,
};
use lazy_static::lazy_static;
use log::info;
use solana_client::{
    nonblocking::{rpc_client::RpcClient, tpu_client::TpuClient},
    rpc_config::RpcSimulateTransactionConfig,
    tpu_client::TpuClientConfig,
};
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, Result as PluginResult,
};
use solana_program::{hash::Hash, message::Message, pubkey::Pubkey};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signature},
    transaction::Transaction,
};
use tokio::{runtime::Runtime, sync::RwLock};

use crate::{config::PluginConfig, pool_position::PoolPosition, utils::read_or_new_keypair};

use super::AccountGet;

/// Number of slots to wait before checking for a confirmed transaction.
static TRANSACTION_CONFIRMATION_PERIOD: u64 = 10;

/// Number of slots to wait before trying to execute a thread while not in the pool.
static THREAD_TIMEOUT_WINDOW: u64 = 8;

/// Number of times to retry a thread simulation.
static MAX_THREAD_SIMULATION_FAILURES: u32 = 5;

/// The constant of the exponential backoff function.
static EXPONENTIAL_BACKOFF_CONSTANT: u32 = 2;

/// TxExecutor
pub struct TxExecutor {
    pub config: PluginConfig,
    pub executable_threads: RwLock<HashMap<Pubkey, ExecutableThreadMetadata>>,
    pub transaction_history: RwLock<HashMap<Pubkey, TransactionMetadata>>,
    pub dropped_threads: AtomicU64,
    pub keypair: Keypair,
}

#[derive(Debug)]
pub struct ExecutableThreadMetadata {
    pub due_slot: u64,
    pub simulation_failures: u32,
}

#[derive(Debug)]
pub struct TransactionMetadata {
    pub slot_sent: u64,
    pub signature: Signature,
}

impl TxExecutor {
    pub fn new(config: PluginConfig) -> Self {
        Self {
            config: config.clone(),
            executable_threads: RwLock::new(HashMap::new()),
            transaction_history: RwLock::new(HashMap::new()),
            dropped_threads: AtomicU64::new(0),
            keypair: read_or_new_keypair(config.keypath),
        }
    }

    pub async fn execute_txs(
        self: Arc<Self>,
        client: Arc<RpcClient>,
        thread_pubkeys: HashSet<Pubkey>,
        slot: u64,
        runtime: Arc<Runtime>,
    ) -> PluginResult<()> {
        // Index the provided threads as executable.
        let mut w_executable_threads = self.executable_threads.write().await;
        thread_pubkeys.iter().for_each(|pubkey| {
            w_executable_threads.insert(
                *pubkey,
                ExecutableThreadMetadata {
                    due_slot: slot,
                    simulation_failures: 0,
                },
            );
        });

        // Drop threads that cross the simulation failure threshold.
        w_executable_threads.retain(|_thread_pubkey, metadata| {
            if metadata.simulation_failures > MAX_THREAD_SIMULATION_FAILURES {
                self.dropped_threads.fetch_add(1, Ordering::Relaxed);
                false
            } else {
                true
            }
        });
        info!(
            "dropped_threads: {:?} executable_threads: {:?}",
            self.dropped_threads.load(Ordering::Relaxed),
            *w_executable_threads
        );
        drop(w_executable_threads);

        // Process retries.
        self.clone()
            .process_retries(client.clone(), slot)
            .await
            .ok();

        // Get self worker's position in the delegate pool.
        let worker_pubkey = Worker::pubkey(self.config.worker_id);
        if let Ok(pool_position) = client.get::<Pool>(&Pool::pubkey(0)).await.map(|pool| {
            let workers = &mut pool.workers.clone();
            PoolPosition {
                current_position: pool
                    .workers
                    .iter()
                    .position(|k| k.eq(&worker_pubkey))
                    .map(|i| i as u64),
                workers: workers.make_contiguous().to_vec().clone(),
            }
        }) {
            // Rotate into the worker pool.
            if pool_position.current_position.is_none() {
                self.clone()
                    .execute_pool_rotate_txs(client.clone(), slot, pool_position.clone())
                    .await
                    .ok();
            }

            // Execute thread transactions.
            self.clone()
                .execute_thread_exec_txs(client.clone(), slot, pool_position, runtime.clone())
                .await
                .ok();
        }

        Ok(())
    }

    async fn process_retries(
        self: Arc<Self>,
        client: Arc<RpcClient>,
        slot: u64,
    ) -> PluginResult<()> {
        // Get transaction signatures and corresponding threads to check.
        struct CheckableTransaction {
            thread_pubkey: Pubkey,
            signature: Signature,
        }
        let r_transaction_history = self.transaction_history.read().await;
        let checkable_transactions = r_transaction_history
            .iter()
            .filter(|(_, metadata)| slot > metadata.slot_sent + TRANSACTION_CONFIRMATION_PERIOD)
            .map(|(pubkey, metadata)| CheckableTransaction {
                thread_pubkey: *pubkey,
                signature: metadata.signature,
            })
            .collect::<Vec<CheckableTransaction>>();
        drop(r_transaction_history);

        // Lookup transaction statuses and track which threads are successful / retriable.
        let mut retriable_threads: HashSet<Pubkey> = HashSet::new();
        let mut successful_threads: HashSet<Pubkey> = HashSet::new();
        for data in checkable_transactions {
            match client
                .get_signature_status_with_commitment(
                    &data.signature,
                    CommitmentConfig::confirmed(),
                )
                .await
            {
                Err(_err) => {}
                Ok(status) => match status {
                    None => {
                        retriable_threads.insert(data.thread_pubkey);
                    }
                    Some(status) => match status {
                        Err(_err) => {
                            retriable_threads.insert(data.thread_pubkey);
                        }
                        Ok(()) => {
                            successful_threads.insert(data.thread_pubkey);
                        }
                    },
                },
            }
        }

        // Requeue retriable threads and drop transactions from history.
        let mut w_transaction_history = self.transaction_history.write().await;
        let mut w_executable_threads = self.executable_threads.write().await;
        for pubkey in successful_threads {
            w_transaction_history.remove(&pubkey);
        }
        for pubkey in retriable_threads {
            w_transaction_history.remove(&pubkey);
            w_executable_threads.insert(
                pubkey,
                ExecutableThreadMetadata {
                    due_slot: slot,
                    simulation_failures: 0,
                },
            );
        }
        info!("transaction_history: {:?}", *w_transaction_history);
        drop(w_executable_threads);
        drop(w_transaction_history);
        Ok(())
    }

    async fn execute_pool_rotate_txs(
        self: Arc<Self>,
        client: Arc<RpcClient>,
        _slot: u64,
        pool_position: PoolPosition,
    ) -> PluginResult<()> {
        let registry = client.get::<Registry>(&Registry::pubkey()).await.unwrap();
        let snapshot_pubkey = Snapshot::pubkey(registry.current_epoch);
        let snapshot_frame_pubkey = SnapshotFrame::pubkey(snapshot_pubkey, self.config.worker_id);
        if let Ok(snapshot) = client.get::<Snapshot>(&snapshot_pubkey).await {
            if let Ok(snapshot_frame) = client.get::<SnapshotFrame>(&snapshot_frame_pubkey).await {
                if let Some(tx) = crate::builders::build_pool_rotation_tx(
                    client.clone(),
                    &self.keypair,
                    pool_position,
                    registry,
                    snapshot,
                    snapshot_frame,
                    self.config.worker_id,
                )
                .await
                {
                    self.clone().execute_tx(&tx).await.ok();
                }
            }
        }
        Ok(())
    }

    async fn execute_thread_exec_txs(
        self: Arc<Self>,
        client: Arc<RpcClient>,
        slot: u64,
        pool_position: PoolPosition,
        runtime: Arc<Runtime>,
    ) -> PluginResult<()> {
        // Get the set of thread pubkeys that are executable.
        // Note we parallelize using rayon because this work is CPU heavy.
        let r_executable_threads = self.executable_threads.read().await;
        let thread_pubkeys =
            if pool_position.current_position.is_none() && !pool_position.workers.is_empty() {
                // This worker is not in the pool. Get pubkeys of threads that are beyond the timeout window.
                r_executable_threads
                    .iter()
                    .filter(|(_pubkey, metadata)| slot > metadata.due_slot + THREAD_TIMEOUT_WINDOW)
                    .filter(|(_pubkey, metadata)| {
                        slot >= metadata.due_slot
                            + EXPONENTIAL_BACKOFF_CONSTANT.pow(metadata.simulation_failures) as u64
                            - 1
                    })
                    .map(|(pubkey, _metadata)| *pubkey)
                    .collect::<Vec<Pubkey>>()
            } else {
                // This worker is in the pool. Get pubkeys executable threads.
                r_executable_threads
                    .iter()
                    .filter(|(_pubkey, metadata)| {
                        slot >= metadata.due_slot
                            + EXPONENTIAL_BACKOFF_CONSTANT.pow(metadata.simulation_failures) as u64
                            - 1
                    })
                    .map(|(pubkey, _metadata)| *pubkey)
                    .collect::<Vec<Pubkey>>()
            };
        drop(r_executable_threads);

        // Process the tasks in parallel.
        // Note we parallelize using tokio because this work is IO heavy (RPC simulation calls).
        let tasks: Vec<_> = thread_pubkeys
            .iter()
            .map(|thread_pubkey| {
                runtime.spawn(
                    self.clone()
                        .process_thread(client.clone(), slot, *thread_pubkey),
                )
            })
            .collect();
        for task in tasks {
            task.await.ok();
        }

        Ok(())
    }

    pub async fn process_thread(
        self: Arc<Self>,
        client: Arc<RpcClient>,
        slot: u64,
        thread_pubkey: Pubkey,
    ) {
        if let Some(tx) = self
            .clone()
            .try_build_thread_exec_tx(client.clone(), thread_pubkey)
            .await
        {
            if self
                .clone()
                .dedupe_tx(slot, thread_pubkey, &tx)
                .await
                .is_ok()
            {
                if self.clone().execute_tx(&tx).await.is_ok() {
                    let mut w_executable_threads = self.executable_threads.write().await;
                    w_executable_threads.remove(&thread_pubkey);
                    drop(w_executable_threads);
                    self.clone().log_tx(slot, thread_pubkey, &tx).await;
                }
            }
        } else {
            let mut w_executable_threads = self.executable_threads.write().await;
            w_executable_threads
                .entry(thread_pubkey)
                .and_modify(|metadata| metadata.simulation_failures += 1);
            drop(w_executable_threads);
        }
    }

    pub async fn try_build_thread_exec_tx(
        self: Arc<Self>,
        client: Arc<RpcClient>,
        thread_pubkey: Pubkey,
    ) -> Option<Transaction> {
        let thread = match client.clone().get::<Thread>(&thread_pubkey).await {
            Err(_err) => {
                return None;
            }
            Ok(thread) => thread,
        };

        crate::builders::build_thread_exec_tx(
            client.clone(),
            &self.keypair,
            thread.clone(),
            thread_pubkey,
            self.config.worker_id,
        )
        .await
    }

    pub async fn dedupe_tx(
        self: Arc<Self>,
        slot: u64,
        thread_pubkey: Pubkey,
        tx: &Transaction,
    ) -> PluginResult<()> {
        let r_transaction_history = self.transaction_history.read().await;
        if let Some(metadata) = r_transaction_history.get(&thread_pubkey) {
            if metadata.signature.eq(&tx.signatures[0]) && metadata.slot_sent.le(&slot) {
                info!("slot: {} thread: {} event: Transaction signature is duplicate of previously submitted transaction", slot, thread_pubkey);
                return Err(GeyserPluginError::Custom(format!("Transaction signature is a duplicate of a previously submitted transaction").into()));
            }
        }
        drop(r_transaction_history);
        Ok(())
    }

    async fn execute_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<()> {
        self.clone().simulate_tx(tx).await?;
        self.clone().submit_tx(tx).await?;
        Ok(())
    }

    async fn simulate_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<Transaction> {
        TPU_CLIENT
            .get()
            .await
            .rpc_client()
            .simulate_transaction_with_config(
                tx,
                RpcSimulateTransactionConfig {
                    replace_recent_blockhash: false,
                    commitment: Some(CommitmentConfig::processed()),
                    ..RpcSimulateTransactionConfig::default()
                },
            )
            .await
            .map_err(|err| {
                GeyserPluginError::Custom(format!("Tx failed simulation: {}", err).into())
            })
            .map(|response| match response.value.err {
                None => Ok(tx.clone()),
                Some(err) => Err(GeyserPluginError::Custom(
                    format!(
                        "Tx failed simulation: {} Logs: {:#?}",
                        err, response.value.logs
                    )
                    .into(),
                )),
            })?
    }

    async fn submit_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<Transaction> {
        if !TPU_CLIENT.get().await.send_transaction(tx).await {
            return Err(GeyserPluginError::Custom(
                "Failed to send transaction".into(),
            ));
        }
        Ok(tx.clone())
    }

    async fn log_tx(self: Arc<Self>, slot: u64, thread_pubkey: Pubkey, tx: &Transaction) {
        let mut w_transaction_history = self.transaction_history.write().await;
        w_transaction_history.insert(
            thread_pubkey,
            TransactionMetadata {
                slot_sent: slot,
                signature: tx.signatures[0],
            },
        );
        drop(w_transaction_history);
    }
}

impl Debug for TxExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tx-executor")
    }
}

/// BlockhashAgnosticHash
trait BlockhashAgnosticHash {
    fn blockhash_agnostic_hash(&self) -> Hash;
}

impl BlockhashAgnosticHash for Message {
    fn blockhash_agnostic_hash(&self) -> Hash {
        Message {
            header: self.header.clone(),
            account_keys: self.account_keys.clone(),
            recent_blockhash: Hash::default(),
            instructions: self.instructions.clone(),
        }
        .hash()
    }
}

static LOCAL_RPC_URL: &str = "http://127.0.0.1:8899";
static LOCAL_WEBSOCKET_URL: &str = "ws://127.0.0.1:8900";

lazy_static! {
    static ref TPU_CLIENT: AsyncOnce<TpuClient> = AsyncOnce::new(async {
        let rpc_client = Arc::new(RpcClient::new_with_commitment(
            LOCAL_RPC_URL.into(),
            CommitmentConfig::processed(),
        ));
        let tpu_client = TpuClient::new(
            rpc_client,
            LOCAL_WEBSOCKET_URL.into(),
            TpuClientConfig::default(),
        )
        .await
        .unwrap();
        tpu_client
    });
}
