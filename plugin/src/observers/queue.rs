use {
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_client::{
        queue::state::{Queue, Trigger, TriggerContext},
        Client as ClockworkClient,
    },
    clockwork_cron::Schedule,
    dashmap::{DashMap, DashSet},
    log::info,
    solana_account_decoder::UiAccountEncoding,
    solana_client::rpc_config::{
        RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig,
    },
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, ReplicaAccountInfo, Result as PluginResult,
    },
    solana_program::{
        clock::Clock,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{account::Account, commitment_config::CommitmentConfig, transaction::Transaction},
    std::{
        collections::hash_map::DefaultHasher,
        fmt::Debug,
        hash::{Hash, Hasher},
        str::FromStr,
        sync::Arc,
    },
    tokio::runtime::Runtime,
};

static COMPUTE_BUDGET_LIMIT: u64 = 1_400_000; // Max number of compute units per transaction
static TRANSACTION_SIZE_LIMIT: usize = 1_232; // Max byte size of a serialized transaction

pub struct QueueObserver {
    // Map from slot numbers to the sysvar clock data for that slot.
    pub clocks: DashMap<u64, Clock>,

    // The set of the queues that are currently crankable (i.e. have a next_instruction)
    pub crankable_queues: DashSet<Pubkey>,

    // Map from unix timestamps to the list of queues scheduled for that moment.
    pub cron_queues: DashMap<i64, DashSet<Pubkey>>,

    // Map from account pubkeys to the set of queues listening for an account update.
    pub listener_queues: DashMap<Pubkey, DashSet<Pubkey>>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl QueueObserver {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            clocks: DashMap::new(),
            crankable_queues: DashSet::new(),
            cron_queues: DashMap::new(),
            listener_queues: DashMap::new(),
            runtime,
        }
    }

    /**
     * Account observers
     */

    pub fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.clocks.retain(|slot, _clock| *slot > confirmed_slot);
            Ok(())
        })
    }

    pub fn handle_updated_clock(self: Arc<Self>, clock: Clock) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.clocks.insert(clock.slot, clock.clone());
            Ok(())
        })
    }

    pub fn handle_updated_account(
        self: Arc<Self>,
        account_pubkey: Pubkey,
        _account_replica: ReplicaAccountInfo,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Move all queues listening to this account into the crankable set.
            this.listener_queues.retain(|pubkey, queue_pubkeys| {
                if account_pubkey.eq(pubkey) {
                    for queue_pubkey in queue_pubkeys.iter() {
                        this.crankable_queues.insert(*queue_pubkey.key());
                    }
                    false
                } else {
                    true
                }
            });

            // TODO This account update could have just been a lamport change (not a data update).
            // TODO To optimize, we need to fetch the queue accounts to verify the data update

            Ok(())
        })
    }

    pub fn handle_updated_queue(
        self: Arc<Self>,
        queue: Queue,
        queue_pubkey: Pubkey,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            info!("Caching queue {:#?} {:#?}", queue_pubkey, queue);

            // Remove queue from crankable set
            this.crankable_queues.remove(&queue_pubkey);

            // If the queue is paused, just return without indexing
            if queue.is_paused {
                return Ok(());
            }

            if queue.next_instruction.is_some() {
                // If the queue has a next instruction, index it as crankable.
                this.crankable_queues.insert(queue_pubkey);
            } else {
                // Otherwise, index the queue according to its trigger type.
                match queue.trigger {
                    Trigger::Account {
                        pubkey: account_pubkey,
                    } => {
                        // Index the queue by its trigger's account pubkey.
                        this.listener_queues
                            .entry(account_pubkey)
                            .and_modify(|v| {
                                v.insert(queue_pubkey);
                            })
                            .or_insert_with(|| {
                                let v = DashSet::new();
                                v.insert(queue_pubkey);
                                v
                            });
                    }
                    Trigger::Cron { schedule } => {
                        // Find a reference timestamp for calculating the queue's upcoming target time.
                        let reference_timestamp = match queue.exec_context {
                            None => queue.created_at.unix_timestamp,
                            Some(exec_context) => match exec_context.trigger_context {
                                TriggerContext::Cron { started_at } => started_at,
                                _ => {
                                    return Err(GeyserPluginError::Custom(
                                        "Invalid exec context".into(),
                                    ))
                                }
                            },
                        };

                        // Index the queue to its target timestamp
                        match next_moment(reference_timestamp, schedule) {
                            None => {} // The queue does not have any upcoming scheduled target time
                            Some(target_timestamp) => {
                                this.cron_queues
                                    .entry(target_timestamp)
                                    .and_modify(|v| {
                                        v.insert(queue_pubkey);
                                    })
                                    .or_insert_with(|| {
                                        let v = DashSet::new();
                                        v.insert(queue_pubkey);
                                        v
                                    });
                            }
                        }
                    }
                    Trigger::Immediate => {
                        this.crankable_queues.insert(queue_pubkey);
                    }
                }
            }

            info!(
                "Queues – crankable: {:#?} cron: {:#?}",
                this.crankable_queues.len(),
                this.cron_queues.len()
            );

            Ok(())
        })
    }

    /**
     * Tx builders
     */

    pub async fn build_crank_txs(
        self: Arc<Self>,
        client: Arc<ClockworkClient>,
        slot: u64,
    ) -> Vec<Transaction> {
        // Get the clock for this slot.
        let clock = match self.clocks.get(&slot) {
            None => return vec![],
            Some(clock) => clock.value().clone(),
        };

        // Index all of the scheduled queues that are now due.
        // Cache retains all queues that are not yet due.
        self.cron_queues.retain(|target_timestamp, queue_pubkeys| {
            let is_due = clock.unix_timestamp >= *target_timestamp;
            if is_due {
                for queue_pubkey_ref in queue_pubkeys.iter() {
                    self.crankable_queues.insert(*queue_pubkey_ref.key());
                }
            }
            !is_due
        });

        // Build the set of crank transactions
        // TODO Use rayon to parallelize this operation
        self.crankable_queues
            .iter()
            .filter_map(|queue_pubkey_ref| {
                self.clone()
                    .build_crank_tx(client.clone(), *queue_pubkey_ref.key())
                    .ok()
            })
            .collect::<Vec<Transaction>>()
    }

    pub fn build_crank_tx(
        self: Arc<Self>,
        client: Arc<ClockworkClient>,
        queue_pubkey: Pubkey,
    ) -> PluginResult<Transaction> {
        // Build the first crank ix
        let queue = client.get::<Queue>(&queue_pubkey).unwrap();
        let blockhash = client
            .get_latest_blockhash()
            .map_err(|_err| GeyserPluginError::Custom("Failed to get latest blockhash".into()))?;
        let worker_pubkey = client.payer_pubkey();

        // Pre-simulate crank ixs and pack into tx
        let mut ixs: Vec<Instruction> =
            vec![self
                .clone()
                .build_crank_ix(client.clone(), queue, worker_pubkey)?];

        // Pre-simulate crank ixs and pack as many as possible into tx.
        let mut tx: Transaction = Transaction::new_with_payer(&vec![], Some(&worker_pubkey));
        let now = std::time::Instant::now();
        loop {
            let mut sim_tx = Transaction::new_with_payer(&ixs, Some(&worker_pubkey));
            sim_tx.sign(&[client.payer()], blockhash);

            // Exit early if tx exceeds Solana's size limit.
            // TODO With QUIC and Transaction v2 lookup tables, Solana will soon support much larger transaction sizes.
            if sim_tx.message_data().len() > TRANSACTION_SIZE_LIMIT {
                info!(
                    "Transaction message exceeded size limit with {} bytes",
                    sim_tx.message_data().len()
                );
                break;
            }

            // Simulate the complete packed tx.
            match client.simulate_transaction_with_config(
                &sim_tx,
                RpcSimulateTransactionConfig {
                    replace_recent_blockhash: true,
                    commitment: Some(CommitmentConfig::processed()),
                    accounts: Some(RpcSimulateTransactionAccountsConfig {
                        encoding: Some(UiAccountEncoding::Base64Zstd),
                        addresses: vec![queue_pubkey.to_string()],
                    }),
                    ..RpcSimulateTransactionConfig::default()
                },
            ) {
                // If there was an error, stop packing and continue with the cranks up until this one.
                Err(_err) => {
                    break;
                }

                // If the simulation was successful, pack the crank ix into the tx.
                Ok(response) => {
                    // If there was an error, then stop packing.
                    if response.value.err.is_some() {
                        break;
                    }

                    // If the compute budget limit was exceeded, then stop packing.
                    if response
                        .value
                        .units_consumed
                        .ge(&Some(COMPUTE_BUDGET_LIMIT))
                    {
                        break;
                    }

                    // Save the simulated tx. It is okay to submit.
                    tx = sim_tx;

                    // Parse the resulting queue account for the next crank ix to simulate.
                    if let Some(ui_accounts) = response.value.accounts {
                        if let Some(Some(ui_account)) = ui_accounts.get(0) {
                            if let Some(account) = ui_account.decode::<Account>() {
                                if let Ok(sim_queue) = Queue::try_from(account.data) {
                                    if sim_queue.next_instruction.is_some() {
                                        ixs.push(self.clone().build_crank_ix(
                                            client.clone(),
                                            sim_queue,
                                            worker_pubkey,
                                        )?);
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        info!("Time spent packing cranks: {:#?}", now.elapsed());

        if tx.message.instructions.len() == 0 {
            return Err(GeyserPluginError::Custom(
                "Transaction has no instructions".into(),
            ));
        }

        Ok(tx)
    }

    fn build_crank_ix(
        self: Arc<Self>,
        client: Arc<ClockworkClient>,
        queue: Queue,
        worker_pubkey: Pubkey,
    ) -> PluginResult<Instruction> {
        // TODO If this queue is an account listener, grab the account and create the data_hash.
        let mut trigger_account_pubkey: Option<Pubkey> = None;
        let mut data_hash: Option<u64> = None;
        match queue.trigger {
            Trigger::Account { pubkey } => {
                // Save the trigger account.
                trigger_account_pubkey = Some(pubkey);

                // Begin computing the data hash of this account.
                let data = client.get_account_data(&pubkey).unwrap();
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);

                // Check the exec context for the prior data hash.
                match queue.exec_context.clone() {
                    None => {
                        // This queue has not begun executing yet.
                        // There is no prior data hash to include in our hash.
                        data_hash = Some(hasher.finish());
                    }
                    Some(exec_context) => {
                        match exec_context.trigger_context {
                            TriggerContext::Account {
                                data_hash: prior_data_hash,
                            } => {
                                // Inject the prior data hash as a seed.
                                prior_data_hash.hash(&mut hasher);
                                data_hash = Some(hasher.finish());
                            }
                            _ => {
                                return Err(GeyserPluginError::Custom("Invalid queue state".into()))
                            }
                        }
                    }
                };
            }
            _ => {}
        }

        // Build the instruction.
        let queue_pubkey = Queue::pubkey(queue.authority, queue.id);
        let inner_ix = queue
            .next_instruction
            .clone()
            .unwrap_or(queue.kickoff_instruction);
        let mut crank_ix = clockwork_client::queue::instruction::queue_crank(
            data_hash,
            queue_pubkey,
            worker_pubkey,
        );

        // Inject the trigger account.
        match trigger_account_pubkey {
            None => {}
            Some(pubkey) => crank_ix.accounts.push(AccountMeta {
                pubkey,
                is_signer: false,
                is_writable: false,
            }),
        }

        // Inject the target program account to the ix.
        crank_ix
            .accounts
            .push(AccountMeta::new_readonly(inner_ix.program_id, false));

        // Inject the worker pubkey as the Clockwork "payer" account
        for acc in inner_ix.clone().accounts {
            let acc_pubkey = if acc.pubkey == clockwork_client::queue::payer::ID {
                worker_pubkey
            } else {
                acc.pubkey
            };
            crank_ix.accounts.push(match acc.is_writable {
                true => AccountMeta::new(acc_pubkey, false),
                false => AccountMeta::new_readonly(acc_pubkey, false),
            })
        }

        Ok(crank_ix)
    }

    /**
     * Runtime helpers
     */

    fn spawn<F: std::future::Future<Output = PluginResult<()>> + Send + 'static>(
        self: &Arc<Self>,
        f: impl FnOnce(Arc<Self>) -> F,
    ) -> PluginResult<()> {
        self.runtime.spawn(f(self.clone()));
        Ok(())
    }
}

impl Debug for QueueObserver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "queue-observer")
    }
}

fn next_moment(after: i64, schedule: String) -> Option<i64> {
    Schedule::from_str(&schedule)
        .unwrap()
        .after(&DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(after, 0),
            Utc,
        ))
        .take(1)
        .next()
        .map(|datetime| datetime.timestamp())
}
