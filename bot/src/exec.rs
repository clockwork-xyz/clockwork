use {
    crate::{bucket::Bucket, cache::TaskCache, env},
    cronos_sdk::account::*,
    solana_client_helpers::Client,
    solana_sdk::pubkey::Pubkey,
    std::{
        sync::{Arc, Mutex, RwLock},
        thread::{self, JoinHandle},
    },
};

const LOOKBACK_WINDOW: i64 = 60 * 15; // Number of seconds to lookback

pub fn execute_tasks(
    client: Arc<Client>,
    cache: Arc<RwLock<TaskCache>>,
    bucket: Arc<Mutex<Bucket>>,
) {
    let time_receiver = cronos_sdk::clock::monitor_time(env::wss_endpoint());
    for ts in time_receiver {
        println!("⏳ Blocktime: {}", ts);
        let tcache = cache.clone();
        let tclient = client.clone();
        let tbucket = bucket.clone();
        thread::spawn(move || execute_tasks_in_lookback_window(tclient, tcache, tbucket, ts));
    }
    execute_tasks(client, cache, bucket)
}

fn execute_tasks_in_lookback_window(
    client: Arc<Client>,
    cache: Arc<RwLock<TaskCache>>,
    bucket: Arc<Mutex<Bucket>>,
    blocktime: i64,
) {
    // Spawn threads to execute tasks in lookback window
    let mut handles = vec![];
    for t in (blocktime - LOOKBACK_WINDOW)..=blocktime {
        let r_cache = cache.read().unwrap();
        r_cache.index.get(&t).and_then(|keys| {
            for key in keys.iter() {
                r_cache.data.get(key).and_then(|task| {
                    handles.push(execute_task(
                        client.clone(),
                        cache.clone(),
                        bucket.clone(),
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
        for h in handles {
            h.join().unwrap();
        }
    }
}

fn execute_task(
    _client: Arc<Client>,
    _cache: Arc<RwLock<TaskCache>>,
    _bucket: Arc<Mutex<Bucket>>,
    _key: Pubkey,
    _task: Task,
) -> JoinHandle<()> {
    thread::spawn(move || {
        // Lock the mutex for this task
        // let mutex = bucket
        //     .lock()
        //     .unwrap()
        //     .get_mutex((key, task.schedule.exec_at));
        // let guard = mutex.try_lock();
        // if guard.is_err() {
        //     return;
        // };
        // let guard = guard.unwrap();

        // // Get accounts
        // let config = Config::pda().0;
        // let fee = Fee::pda(task.daemon).0;

        // // Add accounts to exec instruction
        // let mut ix_exec = cronos_sdk::instruction::task_execute(
        //     config,
        //     task.daemon,
        //     fee,
        //     key,
        //     client.payer_pubkey(),
        // );
        // for acc in task.ix.accounts {
        //     match acc.is_writable {
        //         true => ix_exec.accounts.push(AccountMeta::new(acc.pubkey, false)),
        //         false => ix_exec
        //             .accounts
        //             .push(AccountMeta::new_readonly(acc.pubkey, false)),
        //     }
        // }
        // ix_exec
        //     .accounts
        //     .push(AccountMeta::new_readonly(task.ix.program_id, false));

        // // Sign and submit
        // let res = client.sign_and_submit(
        //     &[ix_exec],
        //     format!("🤖 Executing task: {} {}", key, task.schedule.exec_at).as_str(),
        // );

        // // If exec failed, replicate the task data
        // if res.is_err() {
        //     let err = res.err().unwrap();
        //     println!("❌ {}", err);
        //     let data = client.get_account_data(&key).unwrap();
        //     let task = Task::try_from(data).unwrap();
        //     let mut w_cache = cache.write().unwrap();
        //     w_cache.insert(key, task);
        // }

        // // Drop the mutex
        // drop(guard)
    })
}
