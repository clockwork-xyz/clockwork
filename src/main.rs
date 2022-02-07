use cronos_program::state::Task;
use solana_account_decoder::{UiAccountData, UiAccountEncoding};
use solana_client::{
    pubsub_client::PubsubClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
};
use solana_client_helpers::{ClientResult, RpcClient};
use solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{
    str::FromStr,
    sync::mpsc::{self, Receiver},
    thread,
};

const SOLANA_CLUSTER: &str = "api.devnet.solana.com";
const CHRONOS_PROGRAM_ID: &str = "9cEqpQLV3VGN6mBtFKwheJoreg6BXvyCf6pWWDA1FhRf";

fn main() -> ClientResult<()> {
    // Replicate Cronos tasks to Postgres
    replicate_cronos_tasks(SOLANA_CLUSTER, CHRONOS_PROGRAM_ID);

    // Monitor Solana blocktime
    let mut current_blocktime: i64;
    let blocktime_receiver = monitor_blocktime(SOLANA_CLUSTER);
    for new_blocktime in blocktime_receiver {
        current_blocktime = new_blocktime;
        println!("Current blocktime: {}", current_blocktime);
    }

    Ok(())
}

fn replicate_cronos_tasks(cluster: &'static str, program_id: &'static str) {
    let program_id = Pubkey::from_str(program_id).unwrap();
    let _handle = thread::spawn(move || {
        let (_ws_client, keyed_account_receiver) = PubsubClient::program_subscribe(
            format!("ws://{}", cluster).as_str().into(),
            &program_id,
            Some(RpcProgramAccountsConfig {
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    data_slice: None,
                    commitment: None,
                },
                filters: None,
                with_context: None,
            }),
        )
        .unwrap();

        for keyed_account_response in keyed_account_receiver {
            let keyed_account = keyed_account_response.value;
            let task_account = keyed_account.account.decode::<Account>().unwrap();
            let task_data = Task::try_from_slice(task_account.data).unwrap();

            // let task_data = keyed_account.account.data.decode::<Account>();

            // let task = Task::from::<>(_)
            // let task = Task::try_deserialize(keyed_account.account.data);
            println!("Account: {:?}", keyed_account.pubkey.as_str());
            println!("Accont: {:?}", task_account);
        }
    });
}

fn monitor_blocktime(cluster: &'static str) -> Receiver<i64> {
    let (blocktime_sender, blocktime_receiver) = mpsc::channel::<i64>();
    let _handle = thread::spawn(move || {
        let mut latest_blocktime: i64 = 0;

        // Rpc client
        let rpc_client = RpcClient::new_with_commitment(
            format!("https://{}", cluster).as_str().into(),
            CommitmentConfig::confirmed(),
        );

        // Websocket client
        let (_ws_client, slot_receiver) =
            PubsubClient::slot_subscribe(format!("ws://{}", cluster).as_str().into()).unwrap();

        // Publish updated blocktimes on receiver channel
        for slot_info in slot_receiver {
            let blocktime = rpc_client.get_block_time(slot_info.slot).unwrap();
            if blocktime > latest_blocktime {
                latest_blocktime = blocktime;
                blocktime_sender.send(blocktime).unwrap();
            }
        }
    });
    return blocktime_receiver;
}
