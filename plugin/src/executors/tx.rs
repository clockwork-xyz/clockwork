use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use clockwork_client::{
    network::state::{Pool, Registry, Snapshot, SnapshotFrame, Worker},
    thread::state::Thread,
    Client as ClockworkClient,
};
use dashmap::DashMap;
use log::info;
use rayon::prelude::*;
use solana_client::rpc_config::RpcSimulateTransactionConfig;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, Result as PluginResult,
};
use solana_program::{hash::Hash, message::Message, pubkey::Pubkey};
use solana_sdk::{commitment_config::CommitmentConfig, transaction::Transaction};
use tokio::runtime::Runtime;

use crate::{
    config::PluginConfig, observers::Observers, pool_position::PoolPosition, tpu_client::TpuClient,
};

/// Number of slots to wait before attempting to resend a message.
static MESSAGE_DEDUPE_PERIOD: u64 = 10;

/// Number of slots to wait before trying to execute a thread while not in the pool.
static THREAD_TIMEOUT_WINDOW: u64 = 10;

/// Number of times to retry a thread simulation.
static MAX_THREAD_SIMULATION_FAILURES: u32 = 5;

/// TxExecutor
pub struct TxExecutor {
    pub config: PluginConfig,
    pub message_history: DashMap<Hash, u64>, // Map from message hashes to the slot when that message was sent
    pub simulation_failures: DashMap<Pubkey, u32>,
    pub is_locked: AtomicBool,
}

impl TxExecutor {
    pub fn new(config: PluginConfig) -> Self {
        Self {
            config: config.clone(),
            message_history: DashMap::new(),
            simulation_failures: DashMap::new(),
            is_locked: AtomicBool::new(false),
        }
    }

    pub async fn execute_txs(
        self: Arc<Self>,
        observers: Arc<Observers>,
        client: Arc<ClockworkClient>,
        slot: u64,
        runtime: Arc<Runtime>,
        tpu_client: Arc<TpuClient>,
    ) -> PluginResult<()> {
        // Lock until work is done.
        if self
            .clone()
            .is_locked
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            return Ok(());
        }

        // Drop threads that cross the simulation failure threshold.
        self.clone()
            .simulation_failures
            .retain(|thread_pubkey, failures| {
                if *failures >= MAX_THREAD_SIMULATION_FAILURES {
                    observers.thread.executable_threads.remove(thread_pubkey);
                    false
                } else {
                    true
                }
            });

        // Purge message history that is beyond the dedupe period.
        self.clone()
            .message_history
            .retain(|_msg_hash, msg_slot| *msg_slot >= slot - MESSAGE_DEDUPE_PERIOD);

        // Get self worker's position in the delegate pool.
        let worker_pubkey = Worker::pubkey(self.config.worker_id);
        if let Ok(pool_position) = client.get::<Pool>(&Pool::pubkey(0)).map(|pool| {
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
            self.clone()
                .execute_pool_rotate_txs(
                    client.clone(),
                    slot,
                    pool_position.clone(),
                    tpu_client.clone(),
                )
                .ok();

            // Execute thread transactions.
            self.clone()
                .execute_thread_exec_txs(
                    client.clone(),
                    observers.clone(),
                    slot,
                    pool_position,
                    runtime.clone(),
                    tpu_client,
                )
                .await
                .ok();
        }

        // Release the lock.
        self.clone()
            .is_locked
            .store(false, std::sync::atomic::Ordering::Relaxed);

        Ok(())
    }

    fn execute_pool_rotate_txs(
        self: Arc<Self>,
        client: Arc<ClockworkClient>,
        slot: u64,
        pool_position: PoolPosition,
        tpu_client: Arc<TpuClient>,
    ) -> PluginResult<()> {
        let registry = client.get::<Registry>(&Registry::pubkey()).unwrap();
        let snapshot_pubkey = Snapshot::pubkey(registry.current_epoch);
        let snapshot_frame_pubkey = SnapshotFrame::pubkey(snapshot_pubkey, self.config.worker_id);
        if let Ok(snapshot) = client.get::<Snapshot>(&snapshot_pubkey) {
            if let Ok(snapshot_frame) = client.get::<SnapshotFrame>(&snapshot_frame_pubkey) {
                match crate::builders::build_pool_rotation_tx(
                    client.clone(),
                    pool_position,
                    registry,
                    snapshot,
                    snapshot_frame,
                    self.config.worker_id,
                ) {
                    None => {}
                    Some(tx) => {
                        self.clone().execute_tx(slot, tpu_client, &tx).ok();
                    }
                };
            }
        }
        Ok(())
    }

    async fn execute_thread_exec_txs(
        self: Arc<Self>,
        client: Arc<ClockworkClient>,
        observers: Arc<Observers>,
        slot: u64,
        pool_position: PoolPosition,
        runtime: Arc<Runtime>,
        tpu_client: Arc<TpuClient>,
    ) -> PluginResult<()> {
        // Get the set of thread pubkeys that are executable.
        // Note we parallelize using rayon because this work is CPU heavy.
        let thread_pubkeys =
            if pool_position.current_position.is_none() && !pool_position.workers.is_empty() {
                // This worker is not in the pool. Get pubkeys that are beyond the timeout window.
                observers
                    .thread
                    .executable_threads
                    .par_iter()
                    .filter(|entry| slot > entry.value() + THREAD_TIMEOUT_WINDOW)
                    .map(|entry| *entry.key())
                    .collect::<Vec<Pubkey>>()
            } else {
                // This worker is in the pool. Get pubkeys for executable threads.
                observers
                    .thread
                    .executable_threads
                    .par_iter()
                    .filter(|entry| {
                        // Linear backoff from simulation failures.
                        let failure_count = self
                            .clone()
                            .simulation_failures
                            .get(entry.key())
                            .map(|e| *e.value())
                            .unwrap_or(0);
                        let backoff = ((failure_count * 3) as u64) + entry.value();
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
                runtime.spawn(self.clone().process_thread(
                    client.clone(),
                    observers.clone(),
                    slot,
                    tpu_client.clone(),
                    *thread_pubkey,
                ))
            })
            .collect();
        for task in tasks {
            task.await.ok();
        }

        Ok(())
    }

    pub async fn process_thread(
        self: Arc<Self>,
        client: Arc<ClockworkClient>,
        observers: Arc<Observers>,
        slot: u64,
        tpu_client: Arc<TpuClient>,
        thread_pubkey: Pubkey,
    ) {
        if let Some(tx) = self
            .clone()
            .try_build_thread_exec_tx(client.clone(), thread_pubkey)
            .await
        {
            self.clone().simulation_failures.remove(&thread_pubkey);
            self.clone()
                .execute_tx(slot, tpu_client.clone(), &tx)
                .and_then(|_| {
                    observers.thread.executable_threads.remove(&thread_pubkey);
                    Ok(())
                })
                .ok();
        }
    }

    pub async fn try_build_thread_exec_tx(
        self: Arc<Self>,
        client: Arc<ClockworkClient>,
        thread_pubkey: Pubkey,
    ) -> Option<Transaction> {
        // Get the thread.
        let thread = match client.clone().get::<Thread>(&thread_pubkey) {
            Err(_err) => return None,
            Ok(thread) => thread,
        };

        // Build the thread_exec transaction.
        crate::builders::build_thread_exec_tx(
            client.clone(),
            thread.clone(),
            thread_pubkey,
            self.config.worker_id,
        )
        .await
        .or_else(|| {
            self.clone()
                .simulation_failures
                .entry(thread_pubkey)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            None
        })
    }

    fn execute_tx(
        self: Arc<Self>,
        slot: u64,
        tpu_client: Arc<TpuClient>,
        tx: &Transaction,
    ) -> PluginResult<()> {
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
        self.clone()
            .simulate_tx(tx, tpu_client.clone())
            .and_then(|tx| self.clone().submit_tx(&tx, tpu_client.clone()))
            .and_then(|tx| self.log_tx(slot, tx))
    }

    fn simulate_tx(
        self: Arc<Self>,
        tx: &Transaction,
        tpu_client: Arc<TpuClient>,
    ) -> PluginResult<Transaction> {
        tpu_client
            .rpc_client()
            .simulate_transaction_with_config(
                tx,
                RpcSimulateTransactionConfig {
                    replace_recent_blockhash: true,
                    commitment: Some(CommitmentConfig::processed()),
                    ..RpcSimulateTransactionConfig::default()
                },
            )
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

    fn submit_tx(
        self: Arc<Self>,
        tx: &Transaction,
        tpu_client: Arc<TpuClient>,
    ) -> PluginResult<Transaction> {
        if !tpu_client.send_transaction(tx) {
            return Err(GeyserPluginError::Custom(
                "Failed to send transaction".into(),
            ));
        }
        Ok(tx.clone())
    }

    fn log_tx(self: Arc<Self>, slot: u64, tx: Transaction) -> PluginResult<()> {
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
