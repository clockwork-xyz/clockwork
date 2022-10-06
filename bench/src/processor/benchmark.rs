use {
    crate::cli::CliError,
    clockwork_client::Client,
    solana_client::{
        pubsub_client::PubsubClient,
        rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    },
    solana_sdk::{
        commitment_config::CommitmentConfig,
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
    },
    std::{collections::HashMap, ops::Div},
};

// TODO Redesign the benchmarking test for the crank program

pub fn run(count: u32, parallelism: f32, _recurrence: u32) -> Result<(), CliError> {
    // Setup test
    let config_file = solana_cli_config::CONFIG_FILE.as_ref().unwrap().as_str();
    let solana_config = solana_cli_config::Config::load(config_file).unwrap();
    let payer = read_keypair_file(solana_config.keypair_path).unwrap();
    let client = Client::new(payer, solana_config.json_rpc_url);
    let num_tasks_parallel = (count as f32 * parallelism) as u32;
    let _num_tasks_serial = count - num_tasks_parallel;

    // let mut expected_process_ats = HashMap::<Pubkey, Vec<i64>>::new();
    // let mut actual_process_ats = HashMap::<Pubkey, Vec<i64>>::new();

    // Fund authority account
    let authority = &Keypair::new();
    client
        .airdrop(&authority.pubkey(), LAMPORTS_PER_SOL)
        .unwrap();

    // Create queues for the parallel tasks
    // for i in 0..num_tasks_parallel {
    //     let ix_a = create_queue_ix(
    //         &authority,
    //         recurrence,
    //         &mut expected_process_ats,
    //         i.to_string(),
    //     );
    //     let ix_b = create_task_ix(&authority, i.to_string(), 0);
    //     client
    //         .send_and_confirm(&[ix_a, ix_b], &[authority])
    //         .unwrap();
    // }

    // Create a queue for the serial tasks
    // if num_tasks_serial > 0 {
    //     let ix_a = create_queue_ix(
    //         &authority,
    //         recurrence,
    //         &mut expected_process_ats,
    //         num_tasks_parallel.to_string(),
    //     );

    //     let ixs: &mut Vec<Instruction> = &mut vec![ix_a];

    //     for i in 0..num_tasks_serial {
    //         ixs.push(create_task_ix(
    //             &authority,
    //             num_tasks_parallel.to_string(),
    //             i.into(),
    //         ));
    //     }
    //     client.send_and_confirm(ixs, &[authority]).unwrap();
    // }

    // Collect and report test results
    // let num_expected_events = count * (recurrence + 1);
    // listen_for_events(
    //     num_expected_events,
    //     &expected_process_ats,
    //     &mut actual_process_ats,
    // )?;
    // calculate_and_report_stats(
    //     num_expected_events,
    //     expected_process_ats,
    //     actual_process_ats,
    // )?;

    Ok(())
}

fn _listen_for_events(
    num_expected_events: u32,
    _expected_process_ats: &HashMap<Pubkey, Vec<i64>>,
    _actual_process_ats: &mut HashMap<Pubkey, Vec<i64>>,
) -> Result<(), CliError> {
    let (ws_sub, log_receiver) = PubsubClient::logs_subscribe(
        "ws://localhost:8900/",
        RpcTransactionLogsFilter::Mentions(vec![clockwork_client::queue::ID.to_string()]),
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
                    // if expected_process_ats.contains_key(&event.queue) {
                    //     actual_process_ats
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

fn _calculate_and_report_stats(
    num_expected_events: u32,
    expecteds: HashMap<Pubkey, Vec<i64>>,
    actuals: HashMap<Pubkey, Vec<i64>>,
) -> Result<(), CliError> {
    // Calculate delays
    let mut delays: Vec<i64> = vec![];
    let mut missing = 0;
    for (queue_pubkey, expecteds) in expecteds {
        _get_performance_data(queue_pubkey, expecteds, actuals.clone()).and_then(
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

fn _get_performance_data(
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
