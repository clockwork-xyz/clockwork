use std::{fmt::Debug, sync::Arc};

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
    pub client: Arc<ClockworkClient>, // TODO ClockworkClient and TPUClient can be unified into a single interface
    pub message_history: DashMap<Hash, u64>, // Map from message hashes to the slot when that message was sent
    pub observers: Arc<Observers>,
    pub runtime: Arc<Runtime>,
    pub tpu_client: Arc<TpuClient>,
    pub simulation_failures: DashMap<Pubkey, u32>,
}

impl TxExecutor {
    pub fn new(
        config: PluginConfig,
        client: Arc<ClockworkClient>,
        observers: Arc<Observers>,
        runtime: Arc<Runtime>,
        tpu_client: Arc<TpuClient>,
    ) -> Self {
        Self {
            config: config.clone(),
            client,
            message_history: DashMap::new(),
            observers,
            runtime,
            tpu_client,
            simulation_failures: DashMap::new(),
        }
    }

    pub fn execute_txs(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Get this worker's position in the delegate pool.
            let worker_pubkey = Worker::pubkey(this.config.worker_id);
            let pool_position = this
                .client
                .get::<Pool>(&Pool::pubkey(0))
                .map(|pool| {
                    let workers = &mut pool.workers.clone();
                    PoolPosition {
                        current_position: pool
                            .workers
                            .iter()
                            .position(|k| k.eq(&worker_pubkey))
                            .map(|i| i as u64),
                        workers: workers.make_contiguous().to_vec().clone(),
                    }
                })
                .unwrap();

            info!(
                "simulation_failures: {:?}",
                this.clone().simulation_failures
            );

            // Drop threads that cross the simulation failure threshold.
            this.clone()
                .simulation_failures
                .retain(|thread_pubkey, failures| {
                    if *failures >= MAX_THREAD_SIMULATION_FAILURES {
                        match this.clone().client.clone().get::<Thread>(&thread_pubkey) {
                            Err(_err) => true,
                            Ok(thread) => {
                                this.clone()
                                    .observers
                                    .thread
                                    .drop_thread(thread, *thread_pubkey);
                                false
                            }
                        }
                    } else {
                        true
                    }
                });

            // Purge message history that is beyond the dedupe period.
            this.clone()
                .message_history
                .retain(|_msg_hash, msg_slot| *msg_slot >= slot - MESSAGE_DEDUPE_PERIOD);

            // Rotate into the worker pool.
            this.clone()
                .execute_pool_rotate_txs(slot, pool_position.clone())
                .await
                .ok();

            // Execute thread transactions.
            this.clone()
                .execute_thread_exec_txs(slot, pool_position)
                .await
                .ok();

            Ok(())
        })
    }

    async fn execute_pool_rotate_txs(
        self: Arc<Self>,
        slot: u64,
        pool_position: PoolPosition,
    ) -> PluginResult<()> {
        let registry = self.client.get::<Registry>(&Registry::pubkey()).unwrap();
        let snapshot_pubkey = Snapshot::pubkey(registry.current_epoch);
        let snapshot_frame_pubkey = SnapshotFrame::pubkey(snapshot_pubkey, self.config.worker_id);
        if let Ok(snapshot) = self.client.get::<Snapshot>(&snapshot_pubkey) {
            if let Ok(snapshot_frame) = self.client.get::<SnapshotFrame>(&snapshot_frame_pubkey) {
                match crate::builders::build_pool_rotation_tx(
                    self.client.clone(),
                    pool_position,
                    registry,
                    snapshot,
                    snapshot_frame,
                    self.config.worker_id,
                ) {
                    None => {
                        info!("No pool rotation transaction...");
                    }
                    Some(tx) => {
                        info!("Pool rotation tx: {:?}", tx.signatures[0]);
                        self.clone().execute_tx(slot, &tx).map_err(|err| err).ok();
                    }
                };
            }
        }
        Ok(())
    }

    async fn execute_thread_exec_txs(
        self: Arc<Self>,
        slot: u64,
        pool_position: PoolPosition,
    ) -> PluginResult<()> {
        // Exit early if this worker is not in the delegate pool.
        if pool_position.current_position.is_none() && !pool_position.workers.is_empty() {
            // Attempt executing threads that have been executable for more than the time window.
            self.observers
                .thread
                .executable_threads
                .par_iter()
                .filter(|entry| slot > entry.value() + THREAD_TIMEOUT_WINDOW)
                .filter_map(|entry| {
                    self.clone()
                        .try_build_thread_exec_tx(*entry.key())
                        .map(|tx| (tx, *entry.key()))
                })
                .for_each(|(tx, thread_pubkey)| {
                    self.clone().simulation_failures.remove(&thread_pubkey);
                    self.clone().execute_tx(slot, &tx).map_err(|err| err).ok();
                });
            return Ok(());
        }

        self.observers
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
            .filter_map(|entry| {
                self.clone()
                    .try_build_thread_exec_tx(*entry.key())
                    .map(|tx| (tx, *entry.key()))
            })
            .for_each(|(tx, thread_pubkey)| {
                self.clone().simulation_failures.remove(&thread_pubkey);
                self.clone().execute_tx(slot, &tx).map_err(|err| err).ok();
            });
        Ok(())
    }

    pub fn try_build_thread_exec_tx(self: Arc<Self>, thread_pubkey: Pubkey) -> Option<Transaction> {
        // Get the thread.
        let thread = match self.client.clone().get::<Thread>(&thread_pubkey) {
            Err(_err) => return None,
            Ok(thread) => thread,
        };

        // Build the thread_exec transaction.
        crate::builders::build_thread_exec_tx(
            self.client.clone(),
            thread.clone(),
            thread_pubkey,
            self.config.worker_id,
        )
        .or_else(|| {
            self.clone()
                .simulation_failures
                .entry(thread_pubkey)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            None
        })
    }

    fn execute_tx(self: Arc<Self>, slot: u64, tx: &Transaction) -> PluginResult<()> {
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
            .simulate_tx(tx)
            .and_then(|tx| self.clone().submit_tx(&tx))
            .and_then(|tx| self.log_tx(slot, tx))
    }

    fn simulate_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<Transaction> {
        // TODO Only submit this transaction if the simulated increase in this worker's
        //      Fee account balance is greater than the lamports spent by the worker.

        self.tpu_client
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

    fn submit_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<Transaction> {
        if !self.tpu_client.send_transaction(tx) {
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

    fn spawn<F: std::future::Future<Output = PluginResult<()>> + Send + 'static>(
        self: &Arc<Self>,
        f: impl FnOnce(Arc<Self>) -> F,
    ) -> PluginResult<()> {
        self.runtime.spawn(f(self.clone()));
        Ok(())
    }
}

impl Debug for TxExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tx-executor")
    }
}

/**
 * BlockhashAgnosticHash
 */
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
