use solana_client::rpc_config::RpcSimulateTransactionConfig;

use {
    crate::{config::PluginConfig, observers::Observers, tpu_client::TpuClient},
    clockwork_client::Client as ClockworkClient,
    dashmap::{DashMap, DashSet},
    log::info,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    solana_program::pubkey::Pubkey,
    solana_sdk::{
        commitment_config::CommitmentConfig, signature::Signature, transaction::Transaction,
    },
    std::{
        fmt::Debug,
        hash::{Hash, Hasher},
        sync::Arc,
    },
    tokio::runtime::Runtime,
};

static MAX_RETRIES: u64 = 8; // The maximum number of times a failed tx will be retries before dropping
static TIMEOUT_PERIOD: u64 = 30; // If a signature does not have a status within this many slots, assume failure and retry
static POLLING_INTERVAL: u64 = 3; // Poll for tx statuses on a periodic slot interval. This value must be greater than 0.

/**
 * TxExecutor
 */
pub struct TxExecutor {
    pub config: PluginConfig,
    pub clockwork_client: Arc<ClockworkClient>, // TODO ClockworkClient and TPUClient can be unified into a single interface
    pub observers: Arc<Observers>,
    pub runtime: Arc<Runtime>,
    pub tpu_client: Arc<TpuClient>,
    pub tx_attempts: DashMap<TxType, TxAttempt>,
}

impl TxExecutor {
    pub fn new(
        config: PluginConfig,
        clockwork_client: Arc<ClockworkClient>,
        observers: Arc<Observers>,
        runtime: Arc<Runtime>,
        tpu_client: Arc<TpuClient>,
    ) -> Self {
        Self {
            config: config.clone(),
            clockwork_client,
            observers,
            runtime,
            tpu_client,
            tx_attempts: DashMap::new(),
        }
    }

    pub fn execute_txs(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Rotate the pools
            this.clone().rotate_pools(None, slot).await.ok();

            // Process queues
            this.clone().process_queues(slot).await.ok();

            // Lookup statuses of submitted txs, and retry txs that have timed out or failed
            this.clone().process_tx_history(slot).await.ok();

            // this.process_retry_attempts(retry_attempts, slot).await.ok();

            Ok(())
        })
    }

    async fn rotate_pools(
        self: Arc<Self>,
        prior_attempt: Option<TxAttempt>,
        slot: u64,
    ) -> PluginResult<()> {
        self.observers
            .pool
            .clone()
            .build_rotation_tx(self.clockwork_client.clone(), slot)
            .await
            .and_then(|(target_slot, tx)| {
                match self.execute_tx(prior_attempt, slot, &tx, &TxType::Rotation { target_slot }) {
                    Ok(()) => Ok(()),
                    Err(err) => {
                        info!("Failed to rotate pools: {}", err);
                        Ok(())
                    }
                }
            })
    }

    async fn process_queues(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        // Exit early if we are not in the worker pool.
        let r_pool_positions = self.observers.pool.pool_positions.read().await;
        let pool_position = r_pool_positions.crank_pool_position.clone();
        drop(r_pool_positions);
        if pool_position.current_position.is_none() && !pool_position.workers.is_empty() {
            return Err(GeyserPluginError::Custom(
                "This node is not an authorized worker".into(),
            ));
        }

        self.observers
            .queue
            .clone()
            .build_queue_txs(self.clockwork_client.clone(), slot)
            .await
            .iter()
            .for_each(|(tx, tx_type)| {
                self.clone()
                    .execute_tx(None, slot, tx, tx_type)
                    .map_err(|err| {
                        info!("Failed to process queue: {}", err);
                        err
                    })
                    .ok();
            });

        Ok(())
    }

    async fn process_tx_history(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            let retry_attempts = DashSet::new();

            this.tx_attempts.retain(|_tx_type, tx_attempt| {
                // Exit early the polling interval has not passed
                if confirmed_slot.eq(&tx_attempt.slot)
                    || (confirmed_slot - tx_attempt.slot)
                        .checked_rem(POLLING_INTERVAL)
                        .unwrap_or(0)
                        > 0
                {
                    return true;
                }

                // Lookup the tx signature status
                this.tpu_client
                    .rpc_client()
                    .get_signature_status_with_commitment(
                        &tx_attempt.signature,
                        CommitmentConfig::processed(),
                    )
                    .map(|res| {
                        match res {
                            None => {
                                // Retry txs that have passed the timeout period and do not have a confirmed status
                                info!(
                                    "No confirmation for sig: {} slot: {}",
                                    tx_attempt.signature, confirmed_slot
                                );
                                if confirmed_slot > tx_attempt.slot + TIMEOUT_PERIOD {
                                    retry_attempts.insert(tx_attempt.clone());
                                    false
                                } else {
                                    true
                                }
                            }
                            Some(res) => {
                                // Flag failed txs for retry.
                                // Are there any errors that should not be retried?
                                info!(
                                    "Transaction has status for sig: {} slot: {} result: {:#?}",
                                    tx_attempt.signature, confirmed_slot, res
                                );
                                if res.is_err() {
                                    retry_attempts.insert(tx_attempt.clone());
                                }
                                false
                            }
                        }
                    })
                    .unwrap_or(false)
            });

            this.process_retry_attempts(retry_attempts, confirmed_slot)
                .await?;

            Ok(())
        })
    }

    async fn process_retry_attempts(
        self: Arc<Self>,
        retry_attempts: DashSet<TxAttempt>,
        slot: u64,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Get this node's current position in the crank pool
            // let r_pool_positions = this.observers.pool.pool_positions.read().await;
            // let pool_position = r_pool_positions.crank_pool_position.clone();
            // drop(r_pool_positions);

            // Process all attempts in the retry queue
            for tx_attempt in retry_attempts
                .iter()
                .filter(|tx_attempt| tx_attempt.attempt_count < MAX_RETRIES)
            {
                info!("Processing retry: {:#?}", tx_attempt.key());
                this.tx_attempts.remove(&tx_attempt.tx_type);
                match tx_attempt.tx_type {
                    TxType::Queue {
                        queue_pubkey,
                        crank_hash: _,
                    } => {
                        this.observers
                            .queue
                            .clone()
                            .build_queue_crank_tx(this.clockwork_client.clone(), queue_pubkey)
                            .and_then(|(tx, tx_type)| {
                                this.clone()
                                    .execute_tx(Some(tx_attempt.clone()), slot, &tx, &tx_type)
                                    .map_err(|err| {
                                        info!("Failed to retry queue: {}", err);
                                        err
                                    })
                            })
                            .ok();
                    }
                    TxType::Rotation { target_slot } => {
                        this.clone()
                            .rotate_pools(Some(tx_attempt.clone()), target_slot)
                            .await
                            .ok();
                    }
                }
            }
            Ok(())
        })
    }

    fn execute_tx(
        self: Arc<Self>,
        prior_attempt: Option<TxAttempt>,
        slot: u64,
        tx: &Transaction,
        tx_type: &TxType,
    ) -> PluginResult<()> {
        info!(
            "Executing tx: {:#?} prior_attempt: {:#?}",
            tx_type, prior_attempt
        );

        // Exit early if this is a duplicate attempt
        // if self.tx_attempts.contains_key(tx_type) {
        //     info!("Tx attempts contains duplicate {:#?}...", tx_type);
        //     return Ok(());
        // }

        self.clone()
            .simulate_tx(tx)
            .and_then(|tx| self.clone().submit_tx(&tx))
            // .submit_tx(tx)
            .and_then(|tx| self.log_tx(slot, prior_attempt, tx, *tx_type))
    }

    fn simulate_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<Transaction> {
        info!("Simulating tx... {:#?}", tx);
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
            // .simulate_transaction(tx)
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
        info!("Submitting tx... {:#?}", tx);
        if !self.tpu_client.send_transaction(tx) {
            return Err(GeyserPluginError::Custom(
                "Failed to send transaction".into(),
            ));
        }
        Ok(tx.clone())
    }

    fn log_tx(
        self: Arc<Self>,
        slot: u64,
        prior_attempt: Option<TxAttempt>,
        tx: Transaction,
        tx_type: TxType,
    ) -> PluginResult<()> {
        let sig = tx.signatures[0];
        let tx_attempt = TxAttempt {
            attempt_count: prior_attempt.map_or(0, |prior| prior.attempt_count + 1),
            signature: sig,
            slot,
            tx_type,
        };
        info!(
            "slot: {} sig: {} type: {:#?} attempt: {}",
            slot, sig, tx_attempt.tx_type, tx_attempt.attempt_count
        );
        self.tx_attempts.insert(tx_type, tx_attempt);
        info!("Attempts: {:#?}", self.tx_attempts.len());
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
 * TxAttempt
 */

#[derive(Clone, Copy)]
pub struct TxAttempt {
    pub attempt_count: u64,   // The number of times this tx has been attempted
    pub signature: Signature, // The signature of the last attempt
    pub slot: u64,
    pub tx_type: TxType,
}

impl Hash for TxAttempt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.signature.hash(state);
    }
}

impl PartialEq for TxAttempt {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature
    }
}

impl Debug for TxAttempt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "attempt_count: {} tx_type: {:#?} sig: {} slot: {}",
            self.attempt_count, self.tx_type, self.signature, self.slot
        )
    }
}

impl Eq for TxAttempt {}

/**
 * TxType
 */

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum TxType {
    Queue {
        queue_pubkey: Pubkey,
        crank_hash: u64,
    },
    Rotation {
        target_slot: u64,
    },
}

impl Debug for TxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TxType::Queue {
                queue_pubkey,
                crank_hash,
            } => {
                write!(f, "queue {} hash: {}", queue_pubkey, crank_hash)
            }
            TxType::Rotation { target_slot } => write!(f, "rotation {}", target_slot),
        }
    }
}
