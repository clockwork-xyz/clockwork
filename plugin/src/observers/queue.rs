use {
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_client::{
        crank::state::{ExecContext, InstructionData, Queue, Trigger},
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

// pub struct CrankableQueue {
//     pub pubkey: Pubkey,         // Pubkey of the queue to crank
//     pub data_hash: Option<u64>, // Hash of the queue's data (used for deduping attempts)
// }

// impl Hash for CrankableQueue {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.pubkey.hash(state);
//     }
// }

// impl PartialEq for CrankableQueue {
//     fn eq(&self, other: &Self) -> bool {
//         self.pubkey.eq(&other.pubkey)
//     }
// }

// impl Eq for CrankableQueue {}

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
                            Some(exec_context) => match exec_context {
                                ExecContext::Cron { started_at } => started_at,
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
        // Get the clock for this slot
        let clock = match self.clocks.get(&slot) {
            None => return vec![],
            Some(clock) => clock.value().clone(),
        };

        // Push in all of the scheduled queues that are due.
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
        let mut inner_ix = queue
            .next_instruction
            .clone()
            .unwrap_or(queue.kickoff_instruction);

        // Pre-simulate crank ixs and pack into tx
        let mut ixs: Vec<Instruction> =
            vec![self
                .clone()
                .build_crank_ix(inner_ix, queue_pubkey, worker_pubkey)];
        let mut tx = Transaction::new_with_payer(&ixs, Some(&worker_pubkey));
        tx.sign(&[client.payer()], blockhash);
        let now = std::time::Instant::now();
        let mut continue_packing = true;
        while continue_packing {
            let mut sim_tx = Transaction::new_with_payer(&ixs, Some(&worker_pubkey));
            sim_tx.sign(&[client.payer()], blockhash);

            // Exit early if tx exceeds size limit
            if sim_tx.message_data().len() > TRANSACTION_SIZE_LIMIT {
                info!(
                    "Transaction message exceeded size limit with {} bytes",
                    sim_tx.message_data().len()
                );
                break;
            }

            // Simulate the packed tx
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
                Err(_err) => {
                    continue_packing = false;
                }
                Ok(response) => {
                    if response
                        .value
                        .units_consumed
                        .lt(&Some(COMPUTE_BUDGET_LIMIT))
                    {
                        // This simulated tx is okay to submit
                        tx = sim_tx;

                        // Parse the simulated queue account for another ix to pack
                        if let Some(ui_accounts) = response.value.accounts {
                            if let Some(Some(ui_account)) = ui_accounts.get(0) {
                                if let Some(account) = ui_account.decode::<Account>() {
                                    if let Ok(queue) = Queue::try_from(account.data) {
                                        if let Some(next_instruction) = queue.next_instruction {
                                            ixs.push(self.clone().build_crank_ix(
                                                next_instruction,
                                                queue_pubkey,
                                                worker_pubkey,
                                            ));
                                        } else {
                                            continue_packing = false;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        continue_packing = false;
                    }
                }
            }
        }

        info!("Total packing duration: {:#?}", now.elapsed());

        Ok(tx)
    }

    fn build_crank_ix(
        self: Arc<Self>,
        inner_ix: InstructionData,
        queue_pubkey: Pubkey,
        worker_pubkey: Pubkey,
    ) -> Instruction {
        // Build the queue_crank instruction
        let mut crank_ix =
            clockwork_client::crank::instruction::queue_crank(None, queue_pubkey, worker_pubkey);

        // Program accounts
        crank_ix
            .accounts
            .push(AccountMeta::new_readonly(inner_ix.program_id, false));

        // Inject the worker pubkey as the Clockwork "payer" account
        for acc in inner_ix.clone().accounts {
            let acc_pubkey = if acc.pubkey == clockwork_client::crank::payer::ID {
                worker_pubkey
            } else {
                acc.pubkey
            };
            crank_ix.accounts.push(match acc.is_writable {
                true => AccountMeta::new(acc_pubkey, false),
                false => AccountMeta::new_readonly(acc_pubkey, false),
            })
        }

        crank_ix
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
