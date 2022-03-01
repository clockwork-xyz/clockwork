use cronos_sdk::account::*;
use solana_client_helpers::Client;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

use crate::cache::MutableTaskCache;
use crate::utils::sign_and_submit;
use crate::{cache::TaskCache, utils::monitor_blocktime};

const LOOKBACK_WINDOW: i64 = 120; // Number of seconds to lookback

pub fn execute_tasks(client: Arc<Client>, cache: Arc<RwLock<TaskCache>>) {
    let blocktime_receiver = monitor_blocktime();
    for blocktime in blocktime_receiver {
        println!("⏳ Blocktime: {}", blocktime);
        let tcache = cache.clone();
        let tclient = client.clone();
        thread::spawn(move || execute_tasks_in_lookback_window(tclient, tcache, blocktime));
    }
    execute_tasks(client, cache)
}

fn execute_tasks_in_lookback_window(
    client: Arc<Client>,
    cache: Arc<RwLock<TaskCache>>,
    blocktime: i64,
) {
    // Spawn threads to execute tasks in lookback window
    let mut handles = vec![];
    for t in (blocktime - LOOKBACK_WINDOW)..blocktime {
        let r_cache = cache.read().unwrap();
        r_cache.index.get(&t).and_then(|keys| {
            for key in keys.iter() {
                r_cache.data.get(key).and_then(|task| {
                    handles.push(execute_task(
                        client.clone(),
                        cache.clone(),
                        *key,
                        task.clone(),
                    ));
                    Some(())
                });
            }
            Some(())
        });
    }

    // Join threads
    if !handles.is_empty() {
        println!(
            "Processed {} tasks in blocktime window starting at {}",
            handles.len(),
            blocktime
        );
        for h in handles {
            h.join().unwrap();
        }
    }
}

fn execute_task(
    client: Arc<Client>,
    cache: Arc<RwLock<TaskCache>>,
    key: Pubkey,
    task: Task,
) -> JoinHandle<()> {
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
            let err = res.err().unwrap();
            println!("❌ {}", err);
            let data = client.get_account_data(&key).unwrap();
            let task = Task::try_from(data).unwrap();
            let mut w_cache = cache.write().unwrap();
            w_cache.insert(key, task)
        }
    })
}
