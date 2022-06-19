use {
    crate::{config::PluginConfig, tpu_client::TpuClient},
    bugsnag::Bugsnag,
    cronos_client::{
        scheduler::state::{Queue, Task},
        Client as CronosClient,
    },
    dashmap::{DashMap, DashSet},
    log::info,
    solana_client::rpc_client::RpcClient,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    solana_program::{
        clock::Clock,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{
        commitment_config::CommitmentConfig, signature::Signature, transaction::Transaction,
    },
    solana_transaction_status::{TransactionConfirmationStatus, TransactionStatus},
    std::{collections::HashSet, fmt::Debug, sync::Arc},
    tokio::runtime::{Builder, Runtime},
};

static LOCAL_RPC_URL: &str = "http://127.0.0.1:8899";
static LOCAL_WEBSOCKET_URL: &str = "ws://127.0.0.1:8900";

pub struct Executor {
    // The set of queue pubkeys that can be processed.
    pub actionable_queues: DashSet<Pubkey>,

    // Plugin config values.
    pub config: PluginConfig,

    // Map from exec_at timestamps to the list of queues scheduled
    //  for that moment.
    pub pending_queues: DashMap<i64, DashSet<Pubkey>>,

    // Tokio runtime for processing async tasks.
    pub runtime: Runtime,

    // Map from tx signatures to a (queue pubkey, slot) tuple. The slot
    //  records the confirmed slot at the time the tx was sent.
    pub tx_signatures: DashMap<Signature, (Pubkey, u64)>,

    // Map from slot numbers to the sysvar clock unix_timestamp at that slot.
    pub unix_timestamps: DashMap<u64, i64>,
}

impl Executor {
    pub fn new(config: PluginConfig) -> Self {
        Self {
            actionable_queues: DashSet::new(),
            config: config.clone(),
            pending_queues: DashMap::new(),
            runtime: Builder::new_multi_thread()
                .enable_all()
                .thread_name("cronos-executor")
                .worker_threads(config.worker_threads)
                .max_blocking_threads(config.worker_threads)
                .build()
                .unwrap(),
            tx_signatures: DashMap::new(),
            unix_timestamps: DashMap::new(),
        }
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            info!("Confirmed slot: {}", confirmed_slot);

            // Look for the latest confirmed sysvar unix timestamp
            let mut confirmed_unix_timestamp = None;
            this.unix_timestamps.retain(|slot, unix_timestamp| {
                if *slot == confirmed_slot {
                    confirmed_unix_timestamp = Some(unix_timestamp.clone());
                    return true;
                }
                *slot > confirmed_slot
            });

            // Move all pending queues that are due to the set of actionable queues.
            match confirmed_unix_timestamp {
                Some(confirmed_unix_timestamp) => {
                    // TODO If there are multiple nodes in the delegate pool, can they efficiently
                    //  split up work w/o sending messages to one another?
                    this.pending_queues.retain(|exec_at, queue_pubkeys| {
                        if *exec_at <= confirmed_unix_timestamp {
                            for queue_pubkey in queue_pubkeys.iter() {
                                this.actionable_queues.insert(queue_pubkey.clone());
                            }
                            return false;
                        }
                        true
                    });
                }
                None => (),
            }

            // Process actionable queues
            this.clone()
                .spawn(|this| async move { this.process_queues(confirmed_slot) })?;

            // Confirm signatures
            this.clone()
                .spawn(|this| async move { this.process_tx_signatures(confirmed_slot) })?;

            Ok(())
        })
    }

    pub fn handle_updated_clock(self: Arc<Self>, clock: Clock) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.unix_timestamps
                .insert(clock.slot, clock.unix_timestamp);
            Ok(())
        })
    }

    pub fn handle_updated_queue(
        self: Arc<Self>,
        queue: Queue,
        queue_pubkey: Pubkey,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            info!("Caching queue {:#?}", queue_pubkey);
            match queue.exec_at {
                Some(exec_at) => {
                    this.pending_queues
                        .entry(exec_at)
                        .and_modify(|v| {
                            v.insert(queue_pubkey);
                        })
                        .or_insert_with(|| {
                            let v = DashSet::new();
                            v.insert(queue_pubkey);
                            v
                        });
                }
                None => (),
            };
            Ok(())
        })
    }

    fn process_queues(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Create a new tpu client
            let tpu_client = Arc::new(TpuClient::new(
                this.config.delegate_keypath.clone(),
                LOCAL_RPC_URL.into(),
                LOCAL_WEBSOCKET_URL.into(),
            ));

            // Create a cronos client
            let cronos_client = Arc::new(CronosClient::new(
                this.config.delegate_keypath.clone(),
                LOCAL_RPC_URL.into(),
            ));

            // Error early if the node is not healthy
            tpu_client.rpc_client().get_health().map_err(|_| {
                return GeyserPluginError::Custom("Node is not healthy".into());
            })?;

            // Build a tx for each queue and submit batch via TPU client
            this.actionable_queues
                .iter()
                .filter_map(|queue_pubkey_ref| {
                    let queue_pubkey = *queue_pubkey_ref.key();
                    this.clone()
                        .build_tx(cronos_client.clone(), queue_pubkey)
                        .map_or(None, |tx| Some((queue_pubkey, tx)))
                })
                .collect::<Vec<(Pubkey, Transaction)>>()
                .iter()
                .for_each(|(queue_pubkey, tx)| {
                    if tpu_client.clone().send_transaction(tx) {
                        this.actionable_queues.remove(queue_pubkey);
                        this.tx_signatures
                            .insert(tx.signatures[0], (*queue_pubkey, confirmed_slot));
                    }
                });

            Ok(())
        })
    }

    fn build_tx(
        self: Arc<Self>,
        cronos_client: Arc<CronosClient>,
        queue_pubkey: Pubkey,
    ) -> PluginResult<Transaction> {
        // Get the queue
        let queue = cronos_client.get::<Queue>(&queue_pubkey).unwrap();

        // Build queue_start ix
        let delegate_pubkey = cronos_client.payer_pubkey();
        let queue_start_ix = cronos_client::scheduler::instruction::queue_start(
            delegate_pubkey,
            queue.manager,
            queue_pubkey,
        );

        // Build task_exec ixs
        let mut ixs: Vec<Instruction> = vec![queue_start_ix];
        for i in 0..queue.task_count {
            // Get the task account
            let task_pubkey = Task::pda(queue_pubkey, i).0;
            let task = cronos_client.get::<Task>(&task_pubkey).unwrap();

            // Build ix
            let mut task_exec_ix = cronos_client::scheduler::instruction::task_exec(
                delegate_pubkey,
                queue.manager,
                queue_pubkey,
                task_pubkey,
            );

            // Inject accounts for inner ixs
            let mut acc_dedupe = HashSet::<Pubkey>::new();
            for inner_ix in &task.ixs {
                // Program accounts
                if !acc_dedupe.contains(&inner_ix.program_id) {
                    acc_dedupe.insert(inner_ix.program_id);
                    task_exec_ix
                        .accounts
                        .push(AccountMeta::new_readonly(inner_ix.program_id, false));
                }

                // Other accounts
                for acc in &inner_ix.accounts {
                    if !acc_dedupe.contains(&acc.pubkey) {
                        acc_dedupe.insert(acc.pubkey);

                        // Inject the delegate pubkey as the Cronos "payer" account
                        let mut payer_pubkey = acc.pubkey;
                        if acc.pubkey == cronos_client::scheduler::payer::ID {
                            payer_pubkey = delegate_pubkey;
                        }
                        task_exec_ix.accounts.push(match acc.is_writable {
                            true => AccountMeta::new(payer_pubkey, false),
                            false => AccountMeta::new_readonly(payer_pubkey, false),
                        })
                    }
                }
            }

            // Collect ixs
            ixs.push(task_exec_ix)
        }

        // Pack into tx
        // TODO At what scale must ixs be chunked into separate txs?
        let mut tx = Transaction::new_with_payer(&ixs.clone().to_vec(), Some(&delegate_pubkey));
        tx.sign(
            &[cronos_client.payer()],
            cronos_client.get_latest_blockhash().map_err(|_err| {
                GeyserPluginError::Custom("Failed to get latest blockhash".into())
            })?,
        );
        Ok(tx)
    }

    fn process_tx_signatures(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            let rpc_client = RpcClient::new_with_commitment::<String>(
                LOCAL_RPC_URL.into(),
                CommitmentConfig::confirmed(),
            );
            this.tx_signatures
                .iter()
                .map(|sig_ref| (*sig_ref.key(), sig_ref.value().0, sig_ref.value().1))
                .collect::<Vec<(Signature, Pubkey, u64)>>()
                .chunks(200)
                .flat_map(|chunk| {
                    let only_sigs = &chunk.iter().map(|v| v.0).collect::<Vec<Signature>>();
                    rpc_client
                        .get_signature_statuses(&only_sigs)
                        .expect("status fail")
                        .value
                        .iter()
                        .enumerate()
                        .map(|(i, status)| (status.clone(), chunk[i].0, chunk[i].1, chunk[i].2))
                        .collect::<Vec<(Option<TransactionStatus>, Signature, Pubkey, u64)>>()
                })
                .collect::<Vec<(Option<TransactionStatus>, Signature, Pubkey, u64)>>()
                .iter()
                .for_each(
                    |(status, signature, queue_pubkey, attempted_slot)| match status {
                        Some(status) => {
                            match status.err.clone() {
                                Some(err) => {
                                    info!("Transaction {} failed with error: {}", signature, err);
                                    this.clone().log_error(format!("{:#?}", err));

                                    // TODO Check the error. Should this request be retried?
                                    // TODO Many errors (eg "insufficient funds") should not be retried.

                                    // Naively move the queue pubkey back into the set of actionable queues.
                                    this.tx_signatures.remove(&signature);
                                    this.actionable_queues.insert(*queue_pubkey);
                                }
                                None => {
                                    match status.confirmation_status.clone() {
                                        Some(confirmation_status) => match confirmation_status {
                                            TransactionConfirmationStatus::Confirmed => {
                                                // This signature doesn't need to be checked again
                                                this.tx_signatures.remove(&signature);
                                            }
                                            _ => (), // Wait a little longer
                                        },
                                        None => {
                                            this.clone().retry_if_timeout(
                                                confirmed_slot,
                                                *attempted_slot,
                                                *queue_pubkey,
                                                *signature,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        None => {
                            this.clone().retry_if_timeout(
                                confirmed_slot,
                                *attempted_slot,
                                *queue_pubkey,
                                *signature,
                            );
                        }
                    },
                );
            Ok(())
        })
    }

    fn retry_if_timeout(
        self: Arc<Executor>,
        confirmed_slot: u64,
        attempted_slot: u64,
        queue_pubkey: Pubkey,
        signature: Signature,
    ) {
        // If many slots have passed since the tx was sent, then assume failure
        //  and move the pubkey back into the set of actionable queues.
        if confirmed_slot > attempted_slot + self.config.slot_timeout_threshold {
            self.tx_signatures.remove(&signature);
            self.actionable_queues.insert(queue_pubkey);
        }
    }

    fn spawn<F: std::future::Future<Output = PluginResult<()>> + Send + 'static>(
        self: &Arc<Self>,
        f: impl FnOnce(Arc<Self>) -> F,
    ) -> PluginResult<()> {
        self.runtime.spawn(f(self.clone()));
        Ok(())
    }

    fn log_error(self: Arc<Self>, err_msg: String) {
        match self.config.bugsnag_api_key.clone() {
            Some(api_key) => {
                let mut bugsnag_client = Bugsnag::new(&api_key, env!("CARGO_MANIFEST_DIR"));
                bugsnag_client.set_app_info(
                    Some(env!("CARGO_PKG_VERSION")),
                    Some("development"),
                    Some("rust"),
                );
                bugsnag_client
                    .notify("Error", &err_msg)
                    .severity(bugsnag::Severity::Error);
            }
            None => (),
        }
    }
}

impl Debug for Executor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cronos executor")
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new(PluginConfig::default())
    }
}
