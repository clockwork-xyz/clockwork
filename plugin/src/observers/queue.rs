use {
    super::pool::PoolPosition,
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
            cron_queues: DashMap::new(),
            immediate_queues: DashSet::new(),
            runtime,
        }
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            this.clocks.retain(|slot, _clock| {
                // if confirmed_slot.eq(slot) {
                //     this.confirmed_clock = Some(clock.clone());
                // }
                *slot > confirmed_slot
            });
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

            info!(
                "Indexes: immediate: {:#?}  cron: {:#?}",
                this.immediate_queues, this.cron_queues
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
        pool_position: PoolPosition,
    ) -> Vec<(Pubkey, Transaction)> {
        // TODO Build txs for immediate queues that have not been started
        // TODO Build txs for cron scheduled queues that are due

        // Get this node's current pool position
        // let r_pool_positions = self.pool_positions.read().await;
        // let pool_position = r_pool_positions.scheduler_pool_position.clone();
        // drop(r_pool_positions);

        // Build the set of txs from the actionable queues
        // let txs = self
        //     .actionable_queues
        //     .iter()
        //     .filter_map(|queue_pubkey| {
        //         match self.clone().build_queue_tx(
        //             clockwork_client.clone(),
        //             pool_position.clone(),
        //             queue_pubkey.clone(),
        //             slot,
        //         ) {
        //             Err(_) => None,
        //             Ok(tx) => Some((queue_pubkey.clone(), tx)),
        //         }
        //     })
        //     .collect::<Vec<(Pubkey, Transaction)>>();

        // txs

        vec![]
    }

    pub fn build_queue_tx(
        self: Arc<Self>,
        client: Arc<ClockworkClient>,
        pool_position: PoolPosition,
        queue_pubkey: Pubkey,
        slot: u64,
    ) -> PluginResult<Transaction> {
        // Get the queue
        // let queue = client.get::<Queue>(&queue_pubkey).unwrap();

        // Return none if this queue has no process_at
        // if queue.process_at.is_none() {
        //     return Err(GeyserPluginError::Custom(
        //         "Queue does not have an process_at timestamp".into(),
        //     ));
        // }

        // Exit early this this node is not in the scheduler pool AND
        //  we are still within the queue's grace period.
        // let unix_timestamp = match self.unix_timestamps.get(&slot) {
        //     Some(entry) => *entry.value(),
        //     None => clockwork_client.get_clock().unwrap().unix_timestamp,
        // };
        // if pool_position.current_position.is_none()
        //     && unix_timestamp < queue.process_at.unwrap() + 10
        // {
        //     return Err(GeyserPluginError::Custom(
        //         "This node is not a worker, and the queue is within the grace period".into(),
        //     ));
        // }

        // Setup ixs based on queue's current status
        let worker_pubkey = client.payer_pubkey();
        let mut ixs: Vec<Instruction> = vec![];
        let mut starting_task_id = 0;
        // match queue.status {
        //     QueueStatus::Paused => return Err(GeyserPluginError::Custom("Queue is paused".into())),
        //     QueueStatus::Pending => {
        //         ixs.push(clockwork_client::scheduler::instruction::queue_process(
        //             queue_pubkey,
        //             worker_pubkey,
        //         ));
        //     }
        //     QueueStatus::Processing { task_id } => starting_task_id = task_id,
        // };

        // Build task_exec ixs
        // for i in starting_task_id..queue.task_count {
        //     // Get the task account
        //     let task_pubkey = Task::pubkey(queue_pubkey, i);
        //     let task = clockwork_client.get::<Task>(&task_pubkey).unwrap();

        //     // Build ix
        //     let mut task_exec_ix = clockwork_client::scheduler::instruction::task_exec(
        //         queue_pubkey,
        //         task_pubkey,
        //         worker_pubkey,
        //     );

        //     // Inject accounts for inner ixs
        //     let mut acc_dedupe = HashSet::<Pubkey>::new();
        //     for inner_ix in &task.ixs {
        //         // Program accounts
        //         if !acc_dedupe.contains(&inner_ix.program_id) {
        //             acc_dedupe.insert(inner_ix.program_id);
        //             task_exec_ix
        //                 .accounts
        //                 .push(AccountMeta::new_readonly(inner_ix.program_id, false));
        //         }

        //         // Other accounts
        //         for acc in &inner_ix.accounts {
        //             if !acc_dedupe.contains(&acc.pubkey) {
        //                 acc_dedupe.insert(acc.pubkey);

        //                 // Inject the worker pubkey as the Clockwork "payer" account
        //                 let mut payer_pubkey = acc.pubkey;
        //                 if acc.pubkey == clockwork_client::scheduler::payer::ID {
        //                     payer_pubkey = worker_pubkey;
        //                 }
        //                 task_exec_ix.accounts.push(match acc.is_writable {
        //                     true => AccountMeta::new(payer_pubkey, false),
        //                     false => AccountMeta::new_readonly(payer_pubkey, false),
        //                 })
        //             }
        //         }
        //     }

        //     // Collect ixs
        //     ixs.push(task_exec_ix)
        // }

        // Pack into tx
        // TODO At what scale must ixs be chunked into separate txs?
        let mut tx = Transaction::new_with_payer(&ixs.clone().to_vec(), Some(&worker_pubkey));
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
