use {
    crate::{
        cli::CliError, parser::JsonInstructionData, utils::new_client, utils::sign_and_submit,
    },
    chrono::{prelude::*, Duration},
    cronos_cron::Schedule,
    cronos_sdk::scheduler::events::TaskExecuted,
    cronos_sdk::scheduler::state::{Manager, Queue, Task},
    serde_json::json,
    solana_client::{
        pubsub_client::PubsubClient,
        rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    },
    solana_sdk::{
        borsh, commitment_config::CommitmentConfig, instruction::Instruction,
        native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
    },
    std::{collections::HashMap, ops::Div, str::FromStr},
};

// TODO Refactor this to work with the new Manager/Queue/Task scheduler model

pub fn run(count: u32, parallelism: f32, recurrence: u32) -> Result<(), CliError> {
    // Setup test
    let client = new_client();
    let num_tasks_parallel = (count as f32 * parallelism) as u32;
    let num_tasks_serial = count - num_tasks_parallel;

    let mut authorities: Vec<Keypair> = vec![];
    let mut expected_exec_ats = HashMap::<Pubkey, Vec<i64>>::new();
    let mut actual_exec_ats = HashMap::<Pubkey, Vec<i64>>::new();

    // Create managers
    for _ in 0..(num_tasks_parallel + 1) {
        let authority = Keypair::new();
        let manager_pubkey = Manager::pda(authority.pubkey()).0;
        let ix = cronos_sdk::scheduler::instruction::manager_new(
            authority.pubkey(),
            authority.pubkey(),
            manager_pubkey,
        );
        client
            .airdrop(&authority.pubkey(), LAMPORTS_PER_SOL)
            .unwrap();
        sign_and_submit(&client, &[ix], &authority);
        authorities.push(authority);
    }

    // Create queues for the parallel tasks
    for i in 0..num_tasks_parallel {
        let authority = authorities.get(i as usize).unwrap();
        let ix_a = create_queue_ix(authority, recurrence, &mut expected_exec_ats);
        let ix_b = create_task_ix(authority);
        sign_and_submit(&client, &[ix_a, ix_b], authority);
    }

    // Create a queue for the serial tasks
    let authority = authorities.last().unwrap();
    let ix_a = create_queue_ix(authority, recurrence, &mut expected_exec_ats);
    let ixs: &mut Vec<Instruction> = &mut vec![ix_a];
    for _ in 0..num_tasks_serial {
        ixs.push(create_task_ix(authority))
    }
    sign_and_submit(&client, &ixs.clone(), authority);

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

    // Watch for queue exec events
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

fn create_queue_ix(
    authority: &Keypair,
    recurrence: u32,
    expected_exec: &mut HashMap<Pubkey, Vec<i64>>,
) -> Instruction {
    // Get manager for authority
    let manager_pubkey = Manager::pda(authority.pubkey()).0;

    // Generate ix for new queue account
    let queue_pubkey = Queue::pda(manager_pubkey, 0).0;
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
    let create_queue_ix = cronos_sdk::scheduler::instruction::queue_new(
        authority.pubkey(),
        authority.pubkey(),
        manager_pubkey,
        schedule.clone(),
        queue_pubkey,
    );

    // Index expected exec_at times
    for datetime in Schedule::from_str(&schedule)
        .unwrap()
        .after(&Utc.from_utc_datetime(&Utc::now().naive_utc()))
    {
        expected_exec
            .entry(queue_pubkey)
            .or_insert(Vec::new())
            .push(datetime.timestamp());
    }

    create_queue_ix
}

fn build_memo_ix(manager_pubkey: &Pubkey) -> Instruction {
    let hello_world_memo = json!({
      "program_id": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
      "accounts": [
        {
          "pubkey": manager_pubkey.to_string(),
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

fn create_task_ix(authority: &Keypair) -> Instruction {
    // Get manager for authority
    let manager_pubkey = Manager::pda(authority.pubkey()).0;
    let queue_pubkey = Queue::pda(manager_pubkey, 0).0;

    // let manager = client
    //     .get_account_data(&manager_pubkey)
    //     .map_err(|_err| CliError::AccountNotFound(manager_pubkey.to_string()))
    //     .unwrap();
    // let manager_data = Manager::try_from(manager)
    //     .map_err(|_err| CliError::AccountDataNotParsable(manager_pubkey.to_string()))
    //     .unwrap();

    // Generate PDA for new queue account
    // let queue_pubkey = Queue::pda(manager_pubkey, manager_data.queue_count).0;
    // let now: DateTime<Utc> = Utc::now();
    // let next_minute = now + Duration::minutes(1);
    // let schedule = format!(
    //     "0-{} {} {} {} {} {} {}",
    //     recurrence,
    //     next_minute.minute(),
    //     next_minute.hour(),
    //     next_minute.day(),
    //     next_minute.month(),
    //     next_minute.weekday(),
    //     next_minute.year()
    // );
    // let create_queue_ix = cronos_sdk::scheduler::instruction::queue_new(
    //     owner.pubkey(),
    //     owner.pubkey(),
    //     manager_pubkey,
    //     schedule.clone(),
    //     queue_pubkey,
    // );

    // // Index expected exec_at times
    // for datetime in Schedule::from_str(&schedule)
    //     .unwrap()
    //     .after(&Utc.from_utc_datetime(&Utc::now().naive_utc()))
    // {
    //     expected_exec
    //         .entry(queue_pubkey)
    //         .or_insert(Vec::new())
    //         .push(datetime.timestamp());
    // }

    // Create an task
    let task_pubkey = Task::pda(queue_pubkey, 0).0;
    let memo_ix = build_memo_ix(&manager_pubkey);
    let ix = cronos_sdk::scheduler::instruction::task_new(
        task_pubkey,
        vec![memo_ix],
        authority.pubkey(),
        authority.pubkey(),
        manager_pubkey,
        queue_pubkey,
    );

    ix

    // sign_and_submit(&client, &[create_task_ix], authority);
}

fn calculate_and_report_stats(
    num_expected_events: u32,
    expecteds: HashMap<Pubkey, Vec<i64>>,
    actuals: HashMap<Pubkey, Vec<i64>>,
) -> Result<(), CliError> {
    // Calculate delays
    let mut delays: Vec<i64> = vec![];
    let mut missing = 0;
    for (queue_pubkey, expecteds) in expecteds {
        for i in 0..expecteds.len() {
            let expected = expecteds.get(i).unwrap();
            let actual = actuals.get(&queue_pubkey).unwrap().get(i);
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
