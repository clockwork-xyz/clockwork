use anchor_lang::{prelude::ProgramError, AccountDeserialize};
use cronos_program::state::*;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    pubsub_client::PubsubClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
};
use solana_client_helpers::{Client, ClientResult, RpcClient};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig, instruction::Instruction,
    transaction::Transaction,
};
use std::{
    sync::mpsc::{self, Receiver},
    thread,
};

// const DEVNET_HTTPS_ENDPOINT: &str = "api.devnet.solana.com";
const DEVNET_HTTPS_ENDPOINT: &str = "https://psytrbhymqlkfrhudd.dev.genesysgo.net:8899/";
const DEVNET_WSS_ENDPOINT: &str = "wss://psytrbhymqlkfrhudd.dev.genesysgo.net:8900/";

fn main() -> ClientResult<()> {
    // Replicate Cronos tasks to Postgres
    replicate_cronos_tasks(DEVNET_WSS_ENDPOINT);

    // Monitor Solana blocktime
    let mut current_blocktime: i64;
    let blocktime_receiver = monitor_blocktime(DEVNET_HTTPS_ENDPOINT, DEVNET_WSS_ENDPOINT);
    for new_blocktime in blocktime_receiver {
        current_blocktime = new_blocktime;
        println!("Latest blocktime: {}", current_blocktime);
    }

    Ok(())
}

fn replicate_cronos_tasks(wss_endpoint: &'static str) {
    let _handle = thread::spawn(move || {
        // Websocket client
        let (_ws_client, keyed_account_receiver) = PubsubClient::program_subscribe(
            wss_endpoint.into(),
            &cronos_program::ID,
            Some(RpcProgramAccountsConfig {
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    commitment: Some(CommitmentConfig::processed()),
                    data_slice: None,
                },
                filters: None,
                with_context: None,
            }),
        )
        .unwrap();

        // keyed_account_receiver.recv()
        // keyed_account_receiver.recv_deadline(deadline)

        // Listen for new accounts
        for keyed_account_response in keyed_account_receiver {
            let keyed_account = keyed_account_response.value;
            let account = keyed_account.account.decode::<Account>().unwrap();
            let buf = &mut account.data.as_slice();

            println!("Got account: {:?}", keyed_account);

            // Try deserializing to a task
            let task: Result<Task, ProgramError> = Task::try_deserialize(buf);
            if !task.is_err() {
                let task = task.unwrap();
                // TODO write task to Postgres
            }
        }

        println!("Websocket timed out!");
    });
}

fn monitor_blocktime(https_endpoint: &'static str, wss_endpoint: &'static str) -> Receiver<i64> {
    let (blocktime_sender, blocktime_receiver) = mpsc::channel::<i64>();
    let _handle = thread::spawn(move || {
        let mut latest_blocktime: i64 = 0;

        // Rpc client
        let rpc_client =
            RpcClient::new_with_commitment(https_endpoint.into(), CommitmentConfig::confirmed());

        // Websocket client
        let (_ws_client, slot_receiver) =
            PubsubClient::slot_subscribe(wss_endpoint.into()).unwrap();

        // Listen for new slots
        for slot_info in slot_receiver {
            let blocktime = rpc_client.get_block_time(slot_info.slot).unwrap();

            // Publish updated blocktimes
            if blocktime > latest_blocktime {
                latest_blocktime = blocktime;
                blocktime_sender.send(blocktime).unwrap();
            }
        }
    });
    return blocktime_receiver;
}

fn sign_and_submit(rpc_client: Client, ixs: &[Instruction]) {
    let mut tx = Transaction::new_with_payer(ixs, Some(&rpc_client.payer_pubkey()));
    tx.sign(
        &vec![&rpc_client.payer],
        rpc_client.latest_blockhash().unwrap(),
    );
    let sig = rpc_client.send_and_confirm_transaction(&tx).unwrap();
    println!("Sig: {:?}", sig);
}
