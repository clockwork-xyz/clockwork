use {
    chrono::{prelude::*, Duration},
    cron::Schedule,
    cronos_sdk::scheduler::{
        events::TaskExecuted,
        state::{Daemon, Fee, Task},
    },
    serde_json::json,
    solana_client::{
        pubsub_client::PubsubClient,
        rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    },
    solana_client_helpers::Client,
    solana_sdk::{
        borsh, commitment_config::CommitmentConfig, instruction::Instruction,
        native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
    },
    std::{collections::HashMap, str::FromStr, sync::Arc},
};

use crate::{
    cli::CliError, parser::JsonInstructionData, utils::new_client, utils::sign_and_submit,
};

pub fn run(count: u32, parallelism: f32, recurrence: u32) -> Result<(), CliError> {
    // Setup test
    let client = new_client();
    let num_tasks_parallel = (count as f32 * parallelism) as u32;
    let num_tasks_serial = count - num_tasks_parallel;
    let total_tasks = count * recurrence;

    println!("    total queues: {}", num_tasks_parallel + 1);
    println!("     -- queues in parallel: {}", num_tasks_parallel);
    println!("     -- serial queues: {}", 1);
    println!("tasks in parallel: {}", num_tasks_parallel);
    println!("  tasks in serial: {}\n", num_tasks_serial);

    let mut owners: Vec<Keypair> = vec![];

    let mut expected_exec = HashMap::<Pubkey, Vec<i64>>::new();
    let mut actual_exec = HashMap::<Pubkey, Vec<i64>>::new();

    // Create daemons
    for _ in 0..(num_tasks_parallel + 1) {
        let owner = Keypair::new();
        let daemon_pda = Daemon::pda(owner.pubkey());
        let daemon_addr = daemon_pda.0;
        let fee_pda = Fee::pda(daemon_addr);
        let ix =
            cronos_sdk::scheduler::instruction::daemon_new(daemon_pda, fee_pda, owner.pubkey());
        client.airdrop(&owner.pubkey(), LAMPORTS_PER_SOL).unwrap();
        sign_and_submit(&client, &[ix], &owner);
        owners.push(owner);
    }

    // Schedule parallel tasks
    for i in 0..num_tasks_parallel {
        let owner = owners.get(i as usize).unwrap();
        schedule_memo_task(&client, owner, recurrence, &mut expected_exec);
    }

    // Schedule serial tasks
    let owner = owners.last().unwrap();
    for _ in 0..num_tasks_serial {
        schedule_memo_task(&client, owner, recurrence, &mut expected_exec);
    }

    let included: Vec<String> = vec![cronos_sdk::scheduler::ID.to_string()];
    let url = "ws://localhost:8900/";

    // open web socket to listen for task events
    let (_ws_sub, log_receiver) = PubsubClient::logs_subscribe(
        url,
        RpcTransactionLogsFilter::Mentions(included),
        RpcTransactionLogsConfig {
            commitment: Some(CommitmentConfig::confirmed()),
        },
    )
    .unwrap();

    // parse log data
    let mut counter = 0;
    for log_response in log_receiver {
        let response = log_response.value;
        let logs = response.logs;
        let data = logs.into_iter();

        for string in data {
            match &string[..14] {
                "Program data: " => {
                    // create buffer
                    let mut buffer = Vec::<u8>::new();
                    // decode from string into buffer
                    base64::decode_config_buf(&string[14..], base64::STANDARD, &mut buffer)
                        .unwrap();

                    let task_event =
                        borsh::try_from_slice_unchecked::<TaskExecuted>(&buffer[8..]).unwrap();

                    println!("   task: {}", task_event.task);
                    println!("task ts: {}", task_event.ts);

                    actual_exec
                        .entry(task_event.task)
                        .or_insert(Vec::new())
                        .push(task_event.ts);

                    counter += 1;
                    println!("counter: {}", counter)
                }
                _ => {}
            }
        }

        if counter == total_tasks {
            break;
        }
    }

    stats(expected_exec, actual_exec);

    Ok(())
}

fn build_memo_ix(daemon_pubkey: &Pubkey) -> Instruction {
    let hello_world_memo = json!({
      "program_id": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
      "accounts": [
        {
          "pubkey": daemon_pubkey.to_string(),
          "is_signer": true,
          "is_writable": false
        }
      ],
      "data": [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
    });
    Instruction::try_from(
        &serde_json::from_value::<JsonInstructionData>(hello_world_memo)
            .expect("JSON was not well-formatted"),
    )
    .unwrap()
}

fn schedule_memo_task(
    client: &Arc<Client>,
    owner: &Keypair,
    recurrence: u32,
    expected_exec: &mut HashMap<Pubkey, Vec<i64>>,
) {
    let daemon_pubkey = Daemon::pda(owner.pubkey()).0;
    let memo_ix = build_memo_ix(&daemon_pubkey);
    let daemon_data = client
        .get_account_data(&daemon_pubkey)
        .map_err(|_err| CliError::AccountNotFound(daemon_pubkey.to_string()))
        .unwrap();
    let daemon = Daemon::try_from(daemon_data)
        .map_err(|_err| CliError::AccountDataNotParsable(daemon_pubkey.to_string()))
        .unwrap();
    let task_pda = Task::pda(daemon_pubkey, daemon.task_count);
    let now: DateTime<Utc> = Utc::now();
    let next_minute = now + Duration::minutes(1);
    let schedule = format!(
        "0-{} {} {} {} {} {} {}",
        recurrence - 1,
        next_minute.minute(),
        next_minute.hour(),
        next_minute.day(),
        next_minute.month(),
        next_minute.weekday(),
        next_minute.year()
    );

    let times = Schedule::from_str(&schedule).unwrap();

    //index expected fire times
    for datetime in times.upcoming(Utc) {
        expected_exec
            .entry(task_pda.0)
            .or_insert(Vec::new())
            .push(datetime.timestamp());
    }

    let create_task_ix = cronos_sdk::scheduler::instruction::task_new(
        task_pda,
        daemon_pubkey,
        owner.pubkey(),
        vec![memo_ix],
        schedule,
    );
    sign_and_submit(&client, &[create_task_ix], owner);
}

fn stats(expected_exec: HashMap<Pubkey, Vec<i64>>, actual_exec: HashMap<Pubkey, Vec<i64>>) {
    let mut tasks_avg: Vec<f32> = vec![];
    for (k, v) in expected_exec {
        let mut task_avg: Vec<f32> = vec![];
        println!("               task: {}", k);
        println!("           expected: {}", v.len());
        println!("expected exec times: {:?}", v);

        let actual = actual_exec.get(&k).unwrap();

        println!("             actual: {}", actual.len());
        println!("  actual exec times: {:?}\n", actual);

        for i in 0..(v.len() - 1) {
            task_avg.push((actual[i].abs() - v[i].abs()) as f32);
        }

        // push single task average to tasks_avg
        tasks_avg.push(task_avg.iter().sum::<f32>() as f32 / task_avg.len() as f32);
    }

    let count = tasks_avg.len();
    let mean = tasks_avg.iter().sum::<f32>() as f32 / count as f32;
    let mid = tasks_avg.len() / 2;

    let std_dev = (tasks_avg
        .iter()
        .map(|value| {
            let diff = mean - (*value as f32);

            diff * diff
        })
        .sum::<f32>()
        / count as f32)
        .sqrt();

    println!("mean exec time per task: {:?}", tasks_avg);
    println!("mean: {}", mean);

    tasks_avg.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("median: {}", tasks_avg[mid]);
    println!("standard dev: {}", std_dev);
}
