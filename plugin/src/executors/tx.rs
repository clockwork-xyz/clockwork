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

static MAX_RETRIES: u64 = 2; // The maximum number of times a failed tx will be retries before dropping
static TIMEOUT_PERIOD: u64 = 20; // If a signature does not have a status within this many slots, assume failure and retry
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
    pub tx_dedupe: DashSet<TxType>,
    pub tx_history: DashMap<u64, DashSet<TxAttempt>>,
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
            tx_dedupe: DashSet::new(),
            tx_history: DashMap::new(),
        }
    }

    pub fn execute_txs(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Rotate the pools
            this.clone().rotate_pools(None, slot).await.ok();

            // Proces actionable queues
            this.clone().process_actionable_queues(slot).await.ok();

            // Lookup statuses of submitted txs, and retry txs that have timed out or failed
            let retry_attempts = DashSet::new();
            this.clone()
                .process_tx_history(slot, retry_attempts.clone())
                .await
                .ok();
            this.process_retry_attempts(retry_attempts, slot).await.ok();

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
                match self.execute_tx(prior_attempt, slot, &tx, TxType::Rotation { target_slot }) {
                    Ok(()) => Ok(()),
                    Err(err) => {
                        info!("Failed to rotate pools: {}", err);
                        Ok(())
                    }
                }
            })
    }

    async fn process_actionable_queues(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        let r_pool_positions = self.observers.pool.pool_positions.read().await;
        let pool_position = r_pool_positions.scheduler_pool_position.clone();
        drop(r_pool_positions);

        self.observers
            .queue
            .clone()
            .build_queue_txs(self.clockwork_client.clone(), pool_position, slot)
            .await
            .iter()
            .for_each(|(queue_pubkey, tx)| {
                self.clone()
                    .execute_tx(
                        None,
                        slot,
                        tx,
                        TxType::Queue {
                            queue_pubkey: *queue_pubkey,
                        },
                    )
                    .map_err(|err| {
                        info!("Failed to process actionable queue: {}", err);
                        err
                    })
                    .ok();
            });
        Ok(())
    }

    async fn process_tx_history(
        self: Arc<Self>,
        confirmed_slot: u64,
        retry_attempts: DashSet<TxAttempt>,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Check transaction statuses for every signature in the history
            this.tx_history.iter().for_each(|entry| {
                let attempt_slot = entry.key();
                let tx_attempts = entry.value();

                // Exit early the polling interval has not passed
                if !confirmed_slot.eq(attempt_slot)
                    && (confirmed_slot - attempt_slot)
                        .checked_rem(POLLING_INTERVAL)
                        .unwrap_or(0)
                        > 0
                {
                    return;
                }

                // Lookup tx statuses as a batch
                tx_attempts.clone().iter().for_each(|tx_attempt| {
                    this.tpu_client
                        .rpc_client()
                        .get_signature_status_with_commitment(
                            &tx_attempt.signature,
                            CommitmentConfig::confirmed(),
                        )
                        .map(|opt_res| {
                            match opt_res {
                                None => {
                                    // Retry txs that have passed the timeout period and do not have a confirmed status
                                    if confirmed_slot > attempt_slot + TIMEOUT_PERIOD {
                                        tx_attempts.remove(&tx_attempt.clone());
                                        this.tx_dedupe.remove(&tx_attempt.tx_type);
                                        retry_attempts.insert(tx_attempt.clone());
                                    }
                                }
                                Some(res) => {
                                    tx_attempts.remove(&tx_attempt.clone()); // If a tx has a status, remove it from the history
                                    this.tx_dedupe.remove(&tx_attempt.tx_type);
                                    match res {
                                        Err(_err) => {
                                            // Flag failed txs for retry. Are there any errors that should not be retried?
                                            retry_attempts.insert(tx_attempt.clone());
                                        }
                                        Ok(()) => {}
                                    }
                                }
                            }
                        })
                        .ok();
                });
            });

            // Drop history for all slots where there are no more attempts
            this.tx_history
                .retain(|_, tx_attempts| !tx_attempts.is_empty());

            Ok(())
        })
    }

    async fn process_retry_attempts(
        self: Arc<Self>,
        retry_attempts: DashSet<TxAttempt>,
        slot: u64,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Get this node's current position in the scheduler pool
            let r_pool_positions = this.observers.pool.pool_positions.read().await;
            let pool_position = r_pool_positions.scheduler_pool_position.clone();
            drop(r_pool_positions);

            // Process all attempts in the retry queue
            for tx_attempt in retry_attempts
                .iter()
                .filter(|tx_attempt| tx_attempt.attempt_count < MAX_RETRIES)
            {
                match tx_attempt.tx_type {
                    TxType::Queue { queue_pubkey } => {
                        this.observers
                            .queue
                            .clone()
                            .build_queue_tx(
                                this.clockwork_client.clone(),
                                pool_position.clone(),
                                queue_pubkey,
                                slot,
                            )
                            .and_then(|tx| {
                                this.clone().execute_tx(
                                    Some(tx_attempt.clone()),
                                    slot,
                                    &tx,
                                    TxType::Queue { queue_pubkey },
                                )
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
        tx_type: TxType,
    ) -> PluginResult<()> {
        // Check for dedupes
        if self.tx_dedupe.contains(&tx_type) {
            return Ok(());
        }

        self.clone()
            .simulate_tx(tx)
            .and_then(|tx| self.clone().submit_tx(&tx))
            .and_then(|tx| self.log_tx_attempt(slot, prior_attempt, tx, tx_type))
    }

    fn simulate_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<Transaction> {
        self.tpu_client
            .rpc_client()
            .simulate_transaction(tx)
            .map_err(|err| {
                GeyserPluginError::Custom(format!("Tx failed simulation: {}", err).into())
            })
            .map(|response| match response.value.err {
                None => Ok(tx.clone()),
                Some(err) => Err(GeyserPluginError::Custom(
                    format!("Tx failed simulation: {}", err).into(),
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

    fn log_tx_attempt(
        self: Arc<Self>,
        slot: u64,
        prior_attempt: Option<TxAttempt>,
        tx: Transaction,
        tx_type: TxType,
    ) -> PluginResult<()> {
        let sig = tx.signatures[0];
        let attempt = TxAttempt {
            attempt_count: prior_attempt.map_or(0, |prior| prior.attempt_count + 1),
            signature: sig,
            tx_type,
        };
        info!(
            "slot: {} sig: {} type: {:#?} attempt: {}",
            slot, sig, attempt.tx_type, attempt.attempt_count
        );
        self.tx_dedupe.insert(tx_type);
        self.tx_history
            .entry(slot)
            .and_modify(|tx_attempts| {
                tx_attempts.insert(attempt);
            })
            .or_insert({
                let v = DashSet::new();
                v.insert(attempt);
                v
            });
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
            "attempt_count {}, tx_type: {:#?}",
            self.attempt_count, self.tx_type
        )
    }
}

impl Eq for TxAttempt {}

/**
 * TxType
 */

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum TxType {
    Queue { queue_pubkey: Pubkey },
    Rotation { target_slot: u64 },
}

impl Debug for TxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TxType::Queue { queue_pubkey } => write!(f, "queue {}", queue_pubkey),
            TxType::Rotation { target_slot } => write!(f, "rotation {}", target_slot),
        }
    }
}
