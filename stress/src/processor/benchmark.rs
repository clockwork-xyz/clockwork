use {
    crate::{
        cli::CliError, parser::JsonInstructionData, utils::new_client, utils::sign_and_submit,
    },
    chrono::{prelude::*, Duration},
    cronos_cron::Schedule,
    cronos_sdk::scheduler::events::TaskExecuted,
    cronos_sdk::scheduler::state::{Action, Fee, Queue, Task},
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
    std::{collections::HashMap, ops::Div, str::FromStr, sync::Arc},
};

pub fn run(count: u32, parallelism: f32, recurrence: u32) -> Result<(), CliError> {
    // Setup test
    let client = new_client();
    let num_tasks_parallel = (count as f32 * parallelism) as u32;
    let num_tasks_serial = count - num_tasks_parallel;

    let mut owners: Vec<Keypair> = vec![];
    let mut expected_exec_ats = HashMap::<Pubkey, Vec<i64>>::new();
    let mut actual_exec_ats = HashMap::<Pubkey, Vec<i64>>::new();

    // Create queues
    for _ in 0..(num_tasks_parallel + 1) {
        let owner = Keypair::new();
        let queue_pubkey = Queue::pda(owner.pubkey()).0;
        let fee_pubkey = Fee::pda(queue_pubkey).0;
        let ix =
            cronos_sdk::scheduler::instruction::queue_new(fee_pubkey, owner.pubkey(), queue_pubkey);
        client.airdrop(&owner.pubkey(), LAMPORTS_PER_SOL).unwrap();
        sign_and_submit(&client, &[ix], &owner);
        owners.push(owner);
    }

    // Schedule parallel tasks
    for i in 0..num_tasks_parallel {
        let owner = owners.get(i as usize).unwrap();
        schedule_memo_task(&client, owner, recurrence, &mut expected_exec_ats);
    }

    // Schedule serial tasks
    let owner = owners.last().unwrap();
    for _ in 0..num_tasks_serial {
        schedule_memo_task(&client, owner, recurrence, &mut expected_exec_ats);
    }

    // Collect and report test results
    let num_expected_events = count * (recurrence + 1);
    listen_for_events(num_expected_events, &mut actual_exec_ats)?;
    calculate_and_report_stats(num_expected_events, expected_exec_ats, actual_exec_ats)?;

    Ok(())
}

fn listen_for_events(
    num_expected_events: u32,
    actual_exec_ats: &mut HashMap<Pubkey, Vec<i64>>,
) -> Result<(), CliError> {
    let (ws_sub, log_receiver) = PubsubClient::logs_subscribe(
        "ws://localhost:8900/",
        RpcTransactionLogsFilter::Mentions(vec![cronos_sdk::scheduler::ID.to_string()]),
        RpcTransactionLogsConfig {
            commitment: Some(CommitmentConfig::confirmed()),
        },
    )
    .map_err(|_| CliError::WebsocketError)?;

    // Watch for task exec events
    let mut event_count = 0;

    for log_response in log_receiver {
        let logs = log_response.value.logs.into_iter();
        for log in logs {
            match &log[..14] {
                "Program data: " => {
                    // Decode event from log data
                    let mut buffer = Vec::<u8>::new();
                    base64::decode_config_buf(&log[14..], base64::STANDARD, &mut buffer).unwrap();
                    let event =
                        borsh::try_from_slice_unchecked::<TaskExecuted>(&buffer[8..]).unwrap();
                    actual_exec_ats
                        .entry(event.task)
                        .or_insert(Vec::new())
                        .push(event.ts);
                    event_count += 1;
                }
                _ => {}
            }
        }

        // Exit if we've received the expected number of events
        if event_count == num_expected_events {
            break;
        }
    }

    // TODO: Remove once https://github.com/solana-labs/solana/issues/16102
    //       is addressed. Until then, drop the subscription handle in another
    //       thread so that we deadlock in the other thread as to not block
    //       this thread.
    std::thread::spawn(move || {
        ws_sub.send_unsubscribe().unwrap();
    });

    Ok(())
}

fn build_memo_ix(queue_pubkey: &Pubkey) -> Instruction {
    let hello_world_memo = json!({
      "program_id": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
      "accounts": [
        {
          "pubkey": queue_pubkey.to_string(),
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
    // Get queue for owner
    let queue_pubkey = Queue::pda(owner.pubkey()).0;
    let queue = client
        .get_account_data(&queue_pubkey)
        .map_err(|_err| CliError::AccountNotFound(queue_pubkey.to_string()))
        .unwrap();
    let queue_data = Queue::try_from(queue)
        .map_err(|_err| CliError::AccountDataNotParsable(queue_pubkey.to_string()))
        .unwrap();

    // Generate PDA for new task account
    let task_pubkey = Task::pda(queue_pubkey, queue_data.task_count).0;
    let now: DateTime<Utc> = Utc::now();
    let next_minute = now + Duration::minutes(1);
    let schedule = format!(
        "0-{} {} {} {} {} {} {}",
        recurrence,
        next_minute.minute(),
        next_minute.hour(),
        next_minute.day(),
        next_minute.month(),
        next_minute.weekday(),
        next_minute.year()
    );
    let create_task_ix = cronos_sdk::scheduler::instruction::task_new(
        owner.pubkey(),
        owner.pubkey(),
        queue_pubkey,
        schedule.clone(),
        task_pubkey,
    );

    // Index expected exec_at times
    for datetime in Schedule::from_str(&schedule)
        .unwrap()
        .after(&Utc.from_utc_datetime(&Utc::now().naive_utc()))
    {
        expected_exec
            .entry(task_pubkey)
            .or_insert(Vec::new())
            .push(datetime.timestamp());
    }

    // Create an action
    let action_pda = Action::pda(task_pubkey, 0);
    let memo_ix = build_memo_ix(&queue_pubkey);
    let create_action_ix = cronos_sdk::scheduler::instruction::action_new(
        action_pda,
        vec![memo_ix],
        owner.pubkey(),
        queue_pubkey,
        task_pubkey,
    );

    sign_and_submit(&client, &[create_task_ix, create_action_ix], owner);
}

fn calculate_and_report_stats(
    num_expected_events: u32,
    expecteds: HashMap<Pubkey, Vec<i64>>,
    actuals: HashMap<Pubkey, Vec<i64>>,
) -> Result<(), CliError> {
    // Calculate delays
    let mut delays: Vec<i64> = vec![];
    let mut missing = 0;
    for (task_pubkey, expecteds) in expecteds {
        for i in 0..expecteds.len() {
            let expected = expecteds.get(i).unwrap();
            let actual = actuals.get(&task_pubkey).unwrap().get(i);
            match actual {
                None => missing += 1,
                Some(actual) => {
                    delays.push(actual - expected);
                }
            }
        }
    }
    delays.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Compute stats on delay data
    let mean = delays.iter().sum::<i64>() as f32 / delays.len() as f32;
    let mid = delays.len() / 2;
    let std_dev = delays
        .iter()
        .map(|value| {
            let diff = mean - (*value as f32);
            diff * diff
        })
        .sum::<f32>()
        .div(delays.len() as f32)
        .sqrt();

    // Stdout
    println!("Expected execs: {}", num_expected_events);
    println!("Missing execs: {}", missing);
    println!("Delay (mean): {} sec", mean);
    println!("Delay (median): {} sec", delays[mid]);
    println!("Delay (stddev): {} sec", std_dev);

    Ok(())
}
