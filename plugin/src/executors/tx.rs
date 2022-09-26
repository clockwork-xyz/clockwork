use {
    crate::{config::PluginConfig, observers::Observers, tpu_client::TpuClient},
    clockwork_client::Client as ClockworkClient,
    dashmap::DashMap,
    log::info,
    solana_client::rpc_config::RpcSimulateTransactionConfig,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    solana_program::{hash::Hash, message::Message},
    solana_sdk::{commitment_config::CommitmentConfig, transaction::Transaction},
    std::{fmt::Debug, sync::Arc},
    tokio::runtime::Runtime,
};

static MESSAGE_DEDUPE_PERIOD: u64 = 10; // Number of slots to wait before retrying a message

/**
 * TxExecutor
 */
pub struct TxExecutor {
    pub config: PluginConfig,
    pub clockwork_client: Arc<ClockworkClient>, // TODO ClockworkClient and TPUClient can be unified into a single interface
    pub message_history: DashMap<Hash, u64>, // Map from message hashes to the slot when that message was sent
    pub observers: Arc<Observers>,
    pub runtime: Arc<Runtime>,
    pub tpu_client: Arc<TpuClient>,
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
            message_history: DashMap::new(),
            observers,
            runtime,
            tpu_client,
        }
    }

    pub fn execute_txs(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Rotate worker pools
            this.clone().rotate_pools(slot).await.ok();

            // Crank queues
            this.clone().crank_queues(slot).await.ok();

            // Purge message history that is beyond the dedupe period
            this.message_history
                .retain(|_msg_hash, msg_slot| *msg_slot >= slot - MESSAGE_DEDUPE_PERIOD);

            Ok(())
        })
    }

    async fn rotate_pools(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        self.observers
            .pool
            .clone()
            .build_rotation_tx(self.clockwork_client.clone(), slot)
            .await
            .and_then(|tx| match self.execute_tx(slot, &tx) {
                Ok(()) => Ok(()),
                Err(err) => {
                    info!("Failed to rotate pools: {}", err);
                    Ok(())
                }
            })
    }

    async fn crank_queues(self: Arc<Self>, slot: u64) -> PluginResult<()> {
        // Exit early if we are not in the worker pool.
        let r_pool_positions = self.observers.pool.pool_positions.read().await;
        let crank_pool = r_pool_positions.crank_pool.clone();
        drop(r_pool_positions);
        if crank_pool.current_position.is_none() && !crank_pool.workers.is_empty() {
            return Err(GeyserPluginError::Custom(
                "This node is not an authorized worker".into(),
            ));
        }

        self.observers
            .queue
            .clone()
            .build_crank_txs(self.clockwork_client.clone(), slot)
            .await
            .iter()
            .for_each(|tx| {
                self.clone()
                    .execute_tx(slot, tx)
                    .map_err(|err| {
                        info!("Failed to crank queue: {}", err);
                        err
                    })
                    .ok();
            });

        Ok(())
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
