use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    sync::Arc,
};

use cronos_client::Client as CronosClient;
use dashmap::{DashMap, DashSet};
use log::info;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, Result as PluginResult,
};
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    commitment_config::CommitmentConfig, signature::Signature, transaction::Transaction,
};
use tokio::runtime::Runtime;

use crate::{
    config::PluginConfig, delegate::Delegate, scheduler::Scheduler, tpu_client::TpuClient,
};

static MAX_RETRIES: u64 = 2; // The maximum number of times a failed tx will be retries before dropping
static TIMEOUT_PERIOD: u64 = 20; // If a signature does not have a status within this many slots, assume failure and retry
static POLLING_INTERVAL: u64 = 3; // Poll for tx statuses on a periodic slot interval. This value must be greater than 0.

#[derive(Clone, Copy)]
pub struct TransactionAttempt {
    pub attempt_count: u64,   // The number of times this tx has been attempted
    pub signature: Signature, // The signature of the last attempt
    pub tx_type: TransactionType,
    // TODO Create an abstract representation of each tx. (What args are needed to rebuild the tx?)
    // TODO When retrying, rebuild the tx, simulate, and sign again with a recent blockhash.
}

impl Hash for TransactionAttempt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.signature.hash(state);
    }
}

impl PartialEq for TransactionAttempt {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature
    }
}

impl Eq for TransactionAttempt {}

#[derive(Clone, Copy)]
pub enum TransactionType {
    Queue { pubkey: Pubkey },
    Rotation { slot: u64 },
}

pub struct Executor {
    pub config: PluginConfig,
    pub cronos_client: Arc<CronosClient>, // TODO CronosClient and TPUClient can be unified into a single interface
    pub delegate: Arc<Delegate>,
    pub runtime: Arc<Runtime>,
    pub scheduler: Arc<Scheduler>,
    pub tpu_client: Arc<TpuClient>,
    pub tx_history: DashMap<u64, DashSet<TransactionAttempt>>,
}

impl Executor {
    pub fn new(
        config: PluginConfig,
        cronos_client: Arc<CronosClient>,
        delegate: Arc<Delegate>,
        runtime: Arc<Runtime>,
        scheduler: Arc<Scheduler>,
        tpu_client: Arc<TpuClient>,
    ) -> Self {
        Self {
            config: config.clone(),
            cronos_client,
            delegate,
            runtime,
            scheduler,
            tpu_client,
            tx_history: DashMap::new(),
        }
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Rotate the pools
            this.delegate.clone().handle_confirmed_slot(slot)?;
            this.clone().rotate_pools(None, slot).await.ok();

            // Proces actionable queues
            this.scheduler.clone().handle_confirmed_slot(slot)?;
            this.clone().process_actionable_queues(slot).await.ok();

            // TODO Check on all the signatures in the last X slots
            // TODO For txs that haven't been confirmed in X slots, consider them as "timed out" and retry with a linear backoff
            // TODO Log metrics around tx pending/success/failed counts
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
        prior_attempt: Option<TransactionAttempt>,
        slot: u64,
    ) -> PluginResult<()> {
        self.delegate
            .clone()
            .build_rotation_tx(self.cronos_client.clone(), slot)
            .await
            .and_then(|tx| {
                self.execute_tx(prior_attempt, slot, &tx, TransactionType::Rotation { slot })
            })
    }

    async fn process_actionable_queues(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.scheduler
            .clone()
            .build_queue_txs(self.cronos_client.clone())
            .await
            .iter()
            .for_each(|queue_tx| {
                self.clone()
                    .execute_tx(None, slot, &queue_tx.1, TransactionType::Rotation { slot })
                    .ok();
            });
        Ok(())
    }

    fn execute_tx(
        self: Arc<Self>,
        prior_attempt: Option<TransactionAttempt>,
        slot: u64,
        tx: &Transaction,
        tx_type: TransactionType,
    ) -> PluginResult<()> {
        self.clone()
            .simulate_tx(tx)
            .and_then(|tx| self.clone().submit_tx(&tx))
            .and_then(|tx| self.log_tx_attempt(slot, prior_attempt, tx, tx_type))
    }

    async fn process_tx_history(
        self: Arc<Self>,
        confirmed_slot: u64,
        retry_attempts: DashSet<TransactionAttempt>,
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
                                        retry_attempts.insert(tx_attempt.clone());
                                    }
                                }
                                Some(res) => {
                                    tx_attempts.remove(&tx_attempt.clone()); // If a tx has a status, remove it from the history
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
        retry_attempts: DashSet<TransactionAttempt>,
        slot: u64,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            for tx_attempt in retry_attempts
                .iter()
                .filter(|tx_attempt| tx_attempt.attempt_count < MAX_RETRIES)
            {
                match tx_attempt.tx_type {
                    TransactionType::Queue { pubkey } => {
                        this.scheduler
                            .clone()
                            .build_queue_tx(this.cronos_client.clone(), pubkey)
                            .and_then(|tx| {
                                this.clone().execute_tx(
                                    Some(tx_attempt.clone()),
                                    slot,
                                    &tx,
                                    TransactionType::Queue { pubkey },
                                )
                            })
                            .ok();
                    }
                    TransactionType::Rotation { slot } => {
                        this.clone()
                            .rotate_pools(Some(tx_attempt.clone()), slot)
                            .await
                            .ok();
                    }
                }
            }

            // TODO rebuild tx from arguments
            // TODO simulate tx. drop if simulation fails
            // TODO send tx.
            // TODO log into tx history

            Ok(())
        })
    }

    fn simulate_tx(self: Arc<Self>, tx: &Transaction) -> PluginResult<Transaction> {
        self.tpu_client
            .rpc_client()
            .simulate_transaction(tx)
            .map_err(|_| GeyserPluginError::Custom("Tx failed simulation".into()))
            .map(|response| {
                if response.value.err.is_some() {
                    Err(GeyserPluginError::Custom("Tx failed simulation".into()))
                } else {
                    Ok(tx.clone())
                }
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
        prior_attempt: Option<TransactionAttempt>,
        tx: Transaction,
        tx_type: TransactionType,
    ) -> PluginResult<()> {
        let sig = tx.signatures[0];
        info!("slot: {} sig: {}", slot, sig);
        let attempt = TransactionAttempt {
            attempt_count: prior_attempt.map_or(0, |prior| prior.attempt_count + 1),
            signature: sig,
            tx_type,
        };
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

impl Debug for Executor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Executor")
    }
}
