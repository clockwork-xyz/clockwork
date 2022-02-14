use anchor_lang::prelude::{AccountMeta, Pubkey};
use cronos_sdk::account::*;
use dotenv::dotenv;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    pubsub_client::PubsubClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
};
use solana_client_helpers::{Client, ClientResult, RpcClient};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig, instruction::Instruction,
    signature::read_keypair, transaction::Transaction,
};
use std::{
    env,
    fs::File,
    str::FromStr,
    sync::mpsc::{self, Receiver},
    thread,
};

fn main() -> ClientResult<()> {
    // Load env file
    dotenv().ok();

    // Replicate Cronos tasks to Postgres
    replicate_cronos_tasks();

    // Process pending tasks when Solana blocktime updates
    process_tasks();

    println!("❌ Exiting");
    Ok(())
}

// Blocktime monitoring

fn monitor_blocktime() -> Receiver<i64> {
    let (blocktime_sender, blocktime_receiver) = mpsc::channel::<i64>();
    thread::spawn(move || {
        let mut latest_blocktime: i64 = 0;

        // Rpc client
        let client = new_rpc_client();

        // Websocket client
        let (_ws_client, slot_receiver) =
            PubsubClient::slot_subscribe(env_wss_endpoint().as_str().into()).unwrap();

        // Listen for new slots
        for slot_info in slot_receiver {
            let blocktime = client.get_block_time(slot_info.slot).unwrap();

            // Publish updated blocktimes
            if blocktime > latest_blocktime {
                latest_blocktime = blocktime;
                blocktime_sender.send(blocktime).unwrap();
            }
        }
    });
    return blocktime_receiver;
}

// Task execution

fn process_tasks() {
    let blocktime_receiver = monitor_blocktime();
    for blocktime in blocktime_receiver {
        println!("⏳ Blocktime: {}", blocktime);
        thread::spawn(move || execute_pending_tasks(blocktime));
    }
    process_tasks()
}

fn execute_pending_tasks(blocktime: i64) {
    let mut psql = postgres::Client::connect(env_psql_params().as_str(), postgres::NoTls).unwrap();
    let query = "SELECT * FROM tasks WHERE status = 'pending' AND exec_at <= $1";
    for row in psql.query(query, &[&blocktime]).unwrap() {
        let task = Pubkey::from_str(row.get(0)).unwrap();
        let daemon = Pubkey::from_str(row.get(1)).unwrap();
        thread::spawn(move || execute_task(task, daemon));
    }
}

fn execute_task(pubkey: Pubkey, daemon: Pubkey) {
    let client = new_rpc_client();
    let data = client.get_account_data(&pubkey).unwrap();
    let task = Task::try_from(data).unwrap();
    match task.status {
        TaskStatus::Cancelled | TaskStatus::Done => {
            replicate_task(pubkey, task);
            return;
        }
        TaskStatus::Queued => {
            let config = Config::find_pda().0;
            let fee = Fee::find_pda(daemon).0;
            let mut ix = cronos_sdk::instruction::task_execute(
                config,
                daemon,
                fee,
                pubkey,
                client.payer_pubkey(),
            );
            for acc in task.ix.accounts {
                match acc.is_writable {
                    true => ix.accounts.push(AccountMeta::new(acc.pubkey, false)),
                    false => ix
                        .accounts
                        .push(AccountMeta::new_readonly(acc.pubkey, false)),
                }
            }
            ix.accounts
                .push(AccountMeta::new_readonly(task.ix.program_id, false));
            sign_and_submit(client, &[ix], "Executing task");
        }
    }
}

// Task replication

fn replicate_cronos_tasks() {
    thread::spawn(move || {
        // Websocket client
        let (_ws_client, keyed_account_receiver) = PubsubClient::program_subscribe(
            env_wss_endpoint().as_str().into(),
            &cronos_sdk::ID,
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

        // Listen for new accounts
        for keyed_account_response in keyed_account_receiver {
            let keyed_account = keyed_account_response.value;
            let account = keyed_account.account.decode::<Account>().unwrap();

            // Unwrap task
            let task = Task::try_from(account.data);
            if !task.is_err() {
                let task = task.unwrap();
                replicate_task(Pubkey::from_str(&keyed_account.pubkey).unwrap(), task);
            }
        }

        println!("❌ Websocket connection timed out")
    });
}

fn replicate_task(pubkey: Pubkey, task: Task) {
    println!("💽 Replicate task: {} {}", pubkey, task.status);

    // Build postgres client
    let mut psql = postgres::Client::connect(env_psql_params().as_str(), postgres::NoTls).unwrap();

    // Write task to postgres
    let query = "INSERT INTO tasks 
        (pubkey, daemon, status, exec_at) 
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (pubkey) DO UPDATE SET
        status = EXCLUDED.status,
        exec_at = EXCLUDED.exec_at";
    psql.execute(
        query,
        &[
            &pubkey.to_string(),
            &task.daemon.to_string(),
            &task.status.to_string(),
            &task.schedule.exec_at,
        ],
    )
    .unwrap();
}

// Env

fn env_keypath() -> String {
    env::var("KEYPATH").unwrap()
}

fn env_psql_params() -> String {
    env::var("PSQL_PARAMS").unwrap()
}

fn env_rpc_endpoint() -> String {
    env::var("RPC_ENDPOINT").unwrap()
}

fn env_wss_endpoint() -> String {
    env::var("WSS_ENDPOINT").unwrap()
}

// Helpers

fn new_rpc_client() -> Client {
    let payer = read_keypair(&mut File::open(env_keypath().as_str()).unwrap()).unwrap();
    let client = RpcClient::new_with_commitment(
        env_rpc_endpoint().as_str().into(),
        CommitmentConfig::confirmed(),
    );
    Client { client, payer }
}

fn sign_and_submit(client: Client, ixs: &[Instruction], memo: &str) {
    println!("🤖 {}", memo);
    let mut tx = Transaction::new_with_payer(ixs, Some(&client.payer_pubkey()));
    tx.sign(&vec![&client.payer], client.latest_blockhash().unwrap());
    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    println!("🔏 {:?}", sig);
}
