use cronos_sdk::account::*;
use solana_client_helpers::Client;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use std::sync::{Arc, RwLock};
use std::thread;

use crate::store::MutableTaskStore;
use crate::utils::sign_and_submit;
use crate::{store::TaskStore, utils::monitor_blocktime};

const LOOKBACK_WINDOW: i64 = 10; // Number of seconds to lookback

pub fn execute_tasks(client: Arc<Client>, store: Arc<RwLock<TaskStore>>) {
    let blocktime_receiver = monitor_blocktime();
    for blocktime in blocktime_receiver {
        println!("⏳ Blocktime: {}", blocktime);
        let tstore = store.clone();
        let tclient = client.clone();
        thread::spawn(move || execute_tasks_in_lookback_window(tclient, tstore, blocktime));
    }
    execute_tasks(client, store)
}

fn execute_tasks_in_lookback_window(
    client: Arc<Client>,
    store: Arc<RwLock<TaskStore>>,
    blocktime: i64,
) {
    for t in (blocktime - LOOKBACK_WINDOW)..blocktime {
        let r_store = store.read().unwrap();
        r_store.index.get(&t).and_then(|keys| {
            for key in keys.iter() {
                r_store.data.get(key).and_then(|task| {
                    execute_task(client.clone(), store.clone(), *key, task.clone());
                    Some(())
                });
            }
            Some(())
        });
    }
}

fn execute_task(client: Arc<Client>, store: Arc<RwLock<TaskStore>>, key: Pubkey, task: Task) {
    thread::spawn(move || {
        // Get accounts
        let config = Config::pda().0;
        let fee = Fee::pda(task.daemon).0;

        // Add accounts to exec instruction
        let mut ix_exec = cronos_sdk::instruction::task_execute(
            config,
            task.daemon,
            fee,
            key,
            client.payer_pubkey(),
        );
        for acc in task.ix.accounts {
            match acc.is_writable {
                true => ix_exec.accounts.push(AccountMeta::new(acc.pubkey, false)),
                false => ix_exec
                    .accounts
                    .push(AccountMeta::new_readonly(acc.pubkey, false)),
            }
        }
        ix_exec
            .accounts
            .push(AccountMeta::new_readonly(task.ix.program_id, false));

        // Sign and submit
        let res = sign_and_submit(
            &client,
            &[ix_exec],
            format!("Executing task: {} {}", key, task.daemon).as_str(),
        );

        // If exec failed, replicate the task data
        if res.is_err() {
            let data = client.get_account_data(&key).unwrap();
            let task = Task::try_from(data).unwrap();
            let mut w_store = store.write().unwrap();
            w_store.insert(key, task)
        }
    });
}
