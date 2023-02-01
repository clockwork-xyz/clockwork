use std::{fmt::Debug, sync::Arc};

use async_once::AsyncOnce;
use clockwork_client::{
    network::state::{Pool, Registry, Snapshot, SnapshotFrame, Worker},
    thread::state::Thread,
};
use dashmap::{DashMap, DashSet};
use lazy_static::lazy_static;
use log::info;
use rayon::prelude::*;
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
    commitment_config::CommitmentConfig, signature::Keypair, transaction::Transaction,
};
use tokio::runtime::Runtime;

use crate::{config::PluginConfig, pool_position::PoolPosition, utils::read_or_new_keypair};

use super::AccountGet;

/// Number of slots to wait before attempting to resend a message.
static MESSAGE_DEDUPE_PERIOD: u64 = 8;

/// Number of slots to wait before trying to execute a thread while not in the pool.
static THREAD_TIMEOUT_WINDOW: u64 = 8;

/// Number of times to retry a thread simulation.
static MAX_THREAD_SIMULATION_FAILURES: u32 = 5;

/// Number of slots to wait after simulation failure before retrying again.
static LINEAR_BACKOFF_DURATION: u32 = 4;

/// TxExecutor
pub struct TxExecutor {
    pub config: PluginConfig,
    pub executable_threads: DashMap<Pubkey, ExecutableThreadMetadata>,
    pub message_history: DashMap<Hash, u64>,
    pub keypair: Keypair,
}

#[derive(Debug)]
pub struct ExecutableThreadMetadata {
    pub due_slot: u64,
    pub simulation_failures: u32,
}

impl TxExecutor {
    pub fn new(config: PluginConfig) -> Self {
        Self {
            config: config.clone(),
            executable_threads: DashMap::new(),
            message_history: DashMap::new(),
            keypair: read_or_new_keypair(config.keypath),
        }
    }

    pub async fn execute_txs(
        self: Arc<Self>,
        client: Arc<RpcClient>,
        thread_pubkeys: DashSet<Pubkey>,
        slot: u64,
        runtime: Arc<Runtime>,
    ) -> PluginResult<()> {
        // Index the provided threads as executable.
        thread_pubkeys.par_iter().for_each(|pubkey| {
            self.executable_threads.insert(
                *pubkey,
                ExecutableThreadMetadata {
                    due_slot: slot,
                    simulation_failures: 0,
                },
            );
        });

        info!("executable_threads: {:?}", self.executable_threads);

        // Drop threads that cross the simulation failure threshold.
        self.clone()
            .executable_threads
            .retain(|_thread_pubkey, metadata| {
                metadata.simulation_failures < MAX_THREAD_SIMULATION_FAILURES
            });

        // Purge message history that is beyond the dedupe period.
        self.clone()
            .message_history
            .retain(|_msg_hash, msg_slot| *msg_slot >= slot - MESSAGE_DEDUPE_PERIOD);

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

    async fn execute_pool_rotate_txs(
        self: Arc<Self>,
        client: Arc<RpcClient>,
        slot: u64,
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
                    self.clone().execute_tx(slot, &tx).await.ok();
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
        let thread_pubkeys = if pool_position.current_position.is_none()
            && !pool_position.workers.is_empty()
        {
            // This worker is not in the pool. Get pubkeys of threads that are beyond the timeout window.
            self.executable_threads
                .iter()
                .filter(|entry| slot > entry.value().due_slot + THREAD_TIMEOUT_WINDOW)
                .map(|entry| *entry.key())
                .collect::<Vec<Pubkey>>()
        } else {
            // This worker is in the pool. Get pubkeys executable threads.
            self.executable_threads
                .iter()
                .filter(|entry| {
                    // Linear backoff from simulation failures.
                    let backoff = entry.value().due_slot
                        + ((entry.value().simulation_failures * LINEAR_BACKOFF_DURATION) as u64);
                    slot >= backoff
                })
                .map(|entry| *entry.key())
                .collect::<Vec<Pubkey>>()
        };

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
            if self.clone().execute_tx(slot, &tx).await.is_ok() {
                self.executable_threads.remove(&thread_pubkey);
                // TODO Track the transaction signature
            }
        }
    }

    pub async fn try_build_thread_exec_tx(
        self: Arc<Self>,
        client: Arc<RpcClient>,
        thread_pubkey: Pubkey,
    ) -> Option<Transaction> {
        // Get the thread.
        let thread = match client.clone().get::<Thread>(&thread_pubkey).await {
            Err(_err) => return None,
            Ok(thread) => thread,
        };

        // Build the thread_exec transaction.
        crate::builders::build_thread_exec_tx(
            client.clone(),
            &self.keypair,
            thread.clone(),
            thread_pubkey,
            self.config.worker_id,
        )
        .await
        .or_else(|| {
            self.clone()
                .executable_threads
                .entry(thread_pubkey)
                .and_modify(|metadata| metadata.simulation_failures += 1);
            None
        })
    }

    async fn execute_tx(self: Arc<Self>, slot: u64, tx: &Transaction) -> PluginResult<()> {
        // Exit early if this message was sent recently
        if let Some(entry) = self
            .message_history
            .get(&tx.message().blockhash_agnostic_hash())
        {
            let msg_slot = entry.value();
            if slot < msg_slot + MESSAGE_DEDUPE_PERIOD {
                return Ok(());
            }
        }

        // Simulate and submit the tx
        self.clone().simulate_tx(tx).await?;
        self.clone().submit_tx(tx).await?;
        self.clone().log_tx(slot, tx)
    }

    async fn simulate_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<Transaction> {
        TPU_CLIENT
            .get()
            .await
            .rpc_client()
            .simulate_transaction_with_config(
                tx,
                RpcSimulateTransactionConfig {
                    replace_recent_blockhash: true,
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

    fn log_tx(self: Arc<Self>, slot: u64, tx: &Transaction) -> PluginResult<()> {
        self.message_history
            .insert(tx.message().blockhash_agnostic_hash(), slot);
        let sig = tx.signatures[0];
        info!("slot: {} sig: {}", slot, sig);
        Ok(())
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
