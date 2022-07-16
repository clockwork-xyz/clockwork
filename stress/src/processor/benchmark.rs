use {
    crate::{cli::CliError, parser::JsonInstructionData},
    chrono::{prelude::*, Duration},
    cronos_client::scheduler::state::{Queue, Task},
    cronos_client::Client,
    cronos_cron::Schedule,
    serde_json::json,
    solana_client::{
        pubsub_client::PubsubClient,
        rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    },
    solana_sdk::{
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
    },
    std::{collections::HashMap, ops::Div, str::FromStr},
};

pub fn run(count: u32, parallelism: f32, recurrence: u32) -> Result<(), CliError> {
    // Setup test
    let config_file = solana_cli_config::CONFIG_FILE.as_ref().unwrap().as_str();
    let solana_config = solana_cli_config::Config::load(config_file).unwrap();
    let payer = read_keypair_file(solana_config.keypair_path).unwrap();
    let client = Client::new(payer, solana_config.json_rpc_url);
    let num_tasks_parallel = (count as f32 * parallelism) as u32;
    let num_tasks_serial = count - num_tasks_parallel;

    let mut expected_exec_ats = HashMap::<Pubkey, Vec<i64>>::new();
    let mut actual_exec_ats = HashMap::<Pubkey, Vec<i64>>::new();

    // Fund authority account
    let authority = &Keypair::new();
    client
        .airdrop(&authority.pubkey(), LAMPORTS_PER_SOL)
        .unwrap();

    // TODO Schedule tasks asynchronously

    // Create queues for the parallel tasks
    for i in 0..num_tasks_parallel {
        let ix_a = create_queue_ix(&authority, recurrence, &mut expected_exec_ats, i.into());
        let ix_b = create_task_ix(&authority, i.into(), 0);
        client
            .send_and_confirm(&[ix_a, ix_b], &[authority])
            .unwrap();
    }

    // Create a queue for the serial tasks
    if num_tasks_serial > 0 {
        let ix_a = create_queue_ix(
            &authority,
            recurrence,
            &mut expected_exec_ats,
            num_tasks_parallel.into(),
        );

        let ixs: &mut Vec<Instruction> = &mut vec![ix_a];

        for i in 0..num_tasks_serial {
            ixs.push(create_task_ix(
                &authority,
                num_tasks_parallel.into(),
                i.into(),
            ));
        }
        client.send_and_confirm(ixs, &[authority]).unwrap();
    }

    // Collect and report test results
    let num_expected_events = count * (recurrence + 1);
    listen_for_events(
        num_expected_events,
        &expected_exec_ats,
        &mut actual_exec_ats,
    )?;
    calculate_and_report_stats(num_expected_events, expected_exec_ats, actual_exec_ats)?;

    Ok(())
}

fn listen_for_events(
    num_expected_events: u32,
    _expected_exec_ats: &HashMap<Pubkey, Vec<i64>>,
    _actual_exec_ats: &mut HashMap<Pubkey, Vec<i64>>,
) -> Result<(), CliError> {
    let (ws_sub, log_receiver) = PubsubClient::logs_subscribe(
        "ws://localhost:8900/",
        RpcTransactionLogsFilter::Mentions(vec![cronos_client::scheduler::ID.to_string()]),
        RpcTransactionLogsConfig {
            commitment: Some(CommitmentConfig::confirmed()),
        },
    )
    .map_err(|_| CliError::WebsocketError)?;

    // Watch for task exec events
    let mut _event_count = 0;
    for log_response in log_receiver {
        let logs = log_response.value.logs.into_iter();
        for log in logs {
            match &log[..14] {
                "Program data: " => {
                    // Decode event from log data
                    // let mut buffer = Vec::<u8>::new();
                    // base64::decode_config_buf(&log[14..], base64::STANDARD, &mut buffer).unwrap();
                    // let event =
                    //     borsh::try_from_slice_unchecked::<TaskExecuted>(&buffer[8..]).unwrap();
                    // if expected_exec_ats.contains_key(&event.queue) {
                    //     actual_exec_ats
                    //         .entry(event.queue)
                    //         .or_insert(Vec::new())
                    //         .push(event.ts);
                    //     event_count += 1;
                    // }
                }
                _ => {}
            }
        }

        // Exit if we've received the expected number of events
        if _event_count == num_expected_events {
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
    queue_id: u128,
) -> Instruction {
    // Generate ix for new queue account
    let queue_pubkey = Queue::pubkey(authority.pubkey(), queue_id);
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
    let create_queue_ix = cronos_client::scheduler::instruction::queue_new(
        authority.pubkey(),
        LAMPORTS_PER_SOL,
        0,
        authority.pubkey(),
        queue_pubkey,
        schedule.clone(),
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

fn create_task_ix(authority: &Keypair, queue_id: u128, task_id: u128) -> Instruction {
    let queue_pubkey = Queue::pubkey(authority.pubkey(), queue_id);
    let task_pubkey = Task::pubkey(queue_pubkey, task_id);
    let memo_ix = build_memo_ix(&authority.pubkey());
    cronos_client::scheduler::instruction::task_new(
        authority.pubkey(),
        vec![memo_ix],
        authority.pubkey(),
        queue_pubkey,
        task_pubkey,
    )
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
        get_performance_data(queue_pubkey, expecteds, actuals.clone()).and_then(
            |(missing_data, delay_data)| {
                missing += missing_data;
                delay_data.iter().for_each(|d| delays.push(*d));
                Ok(())
            },
        )?;
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
    println!("Expected tasks: {}", num_expected_events);
    println!("Missing tasks: {}", missing);
    println!("Delay (avg): {} sec", mean);
    println!("Delay (median): {} sec", delays[mid]);
    println!("Delay (std dev): {} sec", std_dev);

    Ok(())
}

fn get_performance_data(
    queue_pubkey: Pubkey,
    expecteds: Vec<i64>,
    actuals: HashMap<Pubkey, Vec<i64>>,
) -> Result<(i32, Vec<i64>), CliError> {
    let mut delays: Vec<i64> = vec![];
    let mut missing = 0;
    let actuals = actuals.get(&queue_pubkey).ok_or(CliError::DataNotFound)?;
    for i in 0..expecteds.len() {
        let expected = expecteds.get(i).unwrap();
        match actuals.get(i) {
            None => missing += 1,
            Some(actual) => delays.push(actual - expected),
        }
    }
    Ok((missing, delays))
}
