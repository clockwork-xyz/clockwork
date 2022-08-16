use {
    super::pool::{PoolPosition, PoolPositions},
    clockwork_client::{
        crank::state::{Exec, Queue},
        Client as ClockworkClient,
    },
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
    std::{collections::HashSet, fmt::Debug, sync::Arc},
    tokio::{runtime::Runtime, sync::RwLock},
};

pub struct QueueObserver {
    pub active_execs: DashMap<Pubkey, Exec>,

    pub queues: DashMap<Pubkey, Queue>,

    // Map from timestamps to the list of queues scheduled for that timestamp.
    pub cron_scheduled_queues: DashMap<i64, DashSet<Pubkey>>,

    // Tokio runtime for processing async tasks.
    pub runtime: Arc<Runtime>,
}

impl QueueObserver {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            // actionable_queues: DashSet::new(),
            active_execs: DashMap::new(),
            cron_scheduled_queues: DashMap::new(),
            queues: DashMap::new(),
            runtime,
        }
    }

    pub fn handle_confirmed_slot(self: Arc<Self>, confirmed_slot: u64) -> PluginResult<()> {
        self.spawn(|this| async move {
            // Lookup the confirmed sysvar unix timestamp
            // let mut confirmed_unix_timestamp = None;
            // this.unix_timestamps.retain(|slot, unix_timestamp| {
            //     if *slot == confirmed_slot {
            //         confirmed_unix_timestamp = Some(unix_timestamp.clone());
            //     }
            //     *slot > confirmed_slot
            // });

            // Move all pending queues that are due to the set of actionable queues.
            // match confirmed_unix_timestamp {
            //     Some(confirmed_unix_timestamp) => {
            //         this.cron_scheduled_queues
            //             .retain(|process_at, queue_pubkeys| {
            //                 if *process_at <= confirmed_unix_timestamp {
            //                     queue_pubkeys.iter().for_each(|pubkey| {
            //                         this.actionable_queues.insert(pubkey.clone());
            //                     });
            //                     false
            //                 } else {
            //                     true
            //                 }
            //             });
            //     }
            //     None => (),
            // }

            Ok(())
        })
    }

    pub fn handle_updated_exec(
        self: Arc<Self>,
        exec: Exec,
        exec_pubkey: Pubkey,
    ) -> PluginResult<()> {
        // TODO

        Ok(())
    }

    pub fn handle_updated_queue(
        self: Arc<Self>,
        queue: Queue,
        queue_pubkey: Pubkey,
    ) -> PluginResult<()> {
        self.spawn(|this| async move {
            info!("Caching queue {:#?}", queue_pubkey);

            // Index the queue by its pubkey
            this.queues.insert(queue_pubkey, queue);

            // TODO Index the queue according to its trigger type

            // Do we need to index queue's according to their upcoming timestamp?
            // And then re-index them once a corresponding exec is created?
            // YES.

            // In this model, queue data rarely updates...
            // Instead, execs get created.
            // Queue lamport balances update...

            // this.actionable_queues.remove(&queue_pubkey);

            // match queue.process_at {
            //     Some(process_at) => {
            //         this.cron_scheduled_queues
            //             .entry(process_at)
            //             .and_modify(|v| {
            //                 v.insert(queue_pubkey);
            //             })
            //             .or_insert_with(|| {
            //                 let v = DashSet::new();
            //                 v.insert(queue_pubkey);
            //                 v
            //             });
            //     }
            //     None => (),
            // };

            Ok(())
        })
    }

    pub async fn build_queue_txs(
        self: Arc<Self>,
        clockwork_client: Arc<ClockworkClient>,
        pool_position: PoolPosition,
        slot: u64,
    ) -> Vec<(Pubkey, Transaction)> {
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
        clockwork_client: Arc<ClockworkClient>,
        pool_position: PoolPosition,
        queue_pubkey: Pubkey,
        slot: u64,
    ) -> PluginResult<Transaction> {
        // Get the queue
        let queue = clockwork_client.get::<Queue>(&queue_pubkey).unwrap();

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
        let worker_pubkey = clockwork_client.payer_pubkey();
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
            &[clockwork_client.payer()],
            clockwork_client.get_latest_blockhash().map_err(|_err| {
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
