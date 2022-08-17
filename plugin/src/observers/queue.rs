use {
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_client::{
        crank::state::{ExecContext, Queue, Trigger},
        Client as ClockworkClient,
    },
    clockwork_cron::Schedule,
    dashmap::{DashMap, DashSet},
    log::info,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    solana_program::{
        clock::Clock,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::transaction::Transaction,
    std::{fmt::Debug, str::FromStr, sync::Arc},
    tokio::runtime::Runtime,
};

pub struct QueueObserver {
    // Map from slot numbers to the sysvar clock data for that slot.
    pub clocks: DashMap<u64, Clock>,

    // The set of the queues that are currently crankable (i.e. have a next_instruction)
    pub crankable_queues: DashSet<Pubkey>,

    // Map from unix timestamps to the list of queues scheduled for that moment.
    pub cron_queues: DashMap<i64, DashSet<Pubkey>>,

    // The set of queues for immediate execution.
    pub immediate_queues: DashSet<Pubkey>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl QueueObserver {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            clocks: DashMap::new(),
            crankable_queues: DashSet::new(),
            cron_queues: DashMap::new(),
            immediate_queues: DashSet::new(),
            runtime,
        }
    }

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

    pub fn handle_updated_queue(
        self: Arc<Self>,
        queue: Queue,
        queue_pubkey: Pubkey,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            info!("Caching queue {:#?} {:#?}", queue_pubkey, queue);

            if queue.next_instruction.is_some() {
                // If the queue has a next instruction, index it as crankable.
                this.crankable_queues.insert(queue_pubkey);
            } else {
                // Index the queue according to its trigger type.
                match queue.trigger {
                    Trigger::Cron { schedule } => {
                        // Find a reference timestamp for calculating the queue's upcoming target time.
                        let reference_timestamp = match queue.exec_context {
                            None => queue.created_at.unix_timestamp,
                            Some(exec_context) => match exec_context {
                                ExecContext::Cron { last_exec_at } => last_exec_at,
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
                        this.immediate_queues.insert(queue_pubkey);
                    }
                }
            }

            info!(
                "Indexes: immediate: {:#?} crankable: {:#?} cron: {:#?}",
                this.immediate_queues, this.crankable_queues, this.cron_queues
            );

            // Do we need to index queues according to their upcoming timestamps?
            // And then re-index them once a new exec context is set?
            // YES.

            Ok(())
        })
    }

    pub async fn build_queue_txs(
        self: Arc<Self>,
        client: Arc<ClockworkClient>,
        slot: u64,
    ) -> Vec<(Pubkey, Transaction)> {
        // TODO Build txs for immediate queues that have not been started
        // TODO Build txs for cron scheduled queues that are due

        info!("Building queue txs...");

        // Get the clock for this slot
        let clock = match self.clocks.get(&slot) {
            None => return vec![],
            Some(clock) => clock.value().clone(),
        };

        // Build the set of queue pubkeys that are executable.
        let crankable_queue_pubkeys = DashSet::<Pubkey>::new();

        // Push in all of the crankable queues.
        self.crankable_queues.retain(|queue_pubkey| {
            crankable_queue_pubkeys.insert(*queue_pubkey);
            false
        });

        // Push in all of the scheduled queues that are due.
        self.cron_queues.retain(|target_timestamp, queue_pubkeys| {
            let is_due = clock.unix_timestamp >= *target_timestamp;
            if is_due {
                for queue_pubkey_ref in queue_pubkeys.iter() {
                    crankable_queue_pubkeys.insert(queue_pubkey_ref.key().clone());
                }
            }
            !is_due
        });

        info!("Actionable queues: {:#?}", crankable_queue_pubkeys);
        info!(
            "Queues immediate: {:#?}  cron: {:#?}",
            self.immediate_queues, self.cron_queues
        );

        // Build the set of exec transactions
        let txs = crankable_queue_pubkeys
            .iter()
            .filter_map(|queue_pubkey_ref| {
                match self
                    .clone()
                    .build_queue_crank_tx(client.clone(), queue_pubkey_ref.key().clone())
                {
                    Err(_) => None,
                    Ok(tx) => Some((queue_pubkey_ref.key().clone(), tx)),
                }
            })
            .collect::<Vec<(Pubkey, Transaction)>>();

        info!("Built the txs: {:#?}", txs);

        txs
    }

    pub fn build_queue_crank_tx(
        self: Arc<Self>,
        client: Arc<ClockworkClient>,
        queue_pubkey: Pubkey,
    ) -> PluginResult<Transaction> {
        // Get the queue
        let queue = client.get::<Queue>(&queue_pubkey).unwrap();

        // Setup ixs based on queue's current status
        let worker_pubkey = client.payer_pubkey();
        let mut ixs: Vec<Instruction> = vec![];

        // Build the queue_crank instruction
        let mut crank_ix =
            clockwork_client::crank::instruction::queue_crank(queue_pubkey, worker_pubkey);

        // Program accounts
        let inner_ix = queue.next_instruction.unwrap_or(queue.first_instruction);
        crank_ix
            .accounts
            .push(AccountMeta::new_readonly(inner_ix.program_id, false));

        // Other accounts
        for acc in inner_ix.accounts {
            // Inject the worker pubkey as the Clockwork "payer" account
            let mut acc_pubkey = acc.pubkey;
            let is_payer = acc_pubkey == clockwork_client::crank::payer::ID;
            if is_payer {
                acc_pubkey = worker_pubkey;
            }
            crank_ix.accounts.push(match acc.is_writable {
                true => AccountMeta::new(acc_pubkey, false),
                false => AccountMeta::new_readonly(acc_pubkey, false),
            })
        }

        ixs.push(crank_ix);

        // Pack into tx
        // TODO At what scale must ixs be chunked into separate txs?
        let mut tx = Transaction::new_with_payer(&ixs, Some(&worker_pubkey));
        tx.sign(
            &[client.payer()],
            client.get_latest_blockhash().map_err(|_err| {
                GeyserPluginError::Custom("Failed to get latest blockhash".into())
            })?,
        );

        Ok(tx)
    }

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
