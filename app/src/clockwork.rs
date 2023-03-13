use std::str::FromStr;

use anchor_lang::{
    prelude::Pubkey, solana_program::instruction::Instruction, Discriminator, InstructionData,
    ToAccountMetas,
};
use clockwork_sdk::state::Thread;
use dotenv_codegen::dotenv;
use solana_client_wasm::{
    solana_sdk::{
        account::Account,
        commitment_config::CommitmentConfig,
        compute_budget::ComputeBudgetInstruction,
        transaction::{Transaction, TransactionError},
    },
    utils::{
        rpc_config::{
            RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcSimulateTransactionConfig,
        },
        rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
    },
    ClientResult, WasmClient,
};
use solana_extra_wasm::{
    account_decoder::{UiAccount, UiAccountEncoding},
    transaction_status::UiTransactionEncoding,
};

pub async fn get_threads() -> Vec<(Thread, Account)> {
    const HELIUS_API_KEY: &str = dotenv!("HELIUS_API_KEY");
    let url = format!("https://rpc.helius.xyz/?api-key={}", HELIUS_API_KEY);
    let helius_rpc_endpoint = url.as_str();
    let client = WasmClient::new(helius_rpc_endpoint);
    // let client = WasmClient::new("http://74.118.139.244:8899");

    let accounts = client
        .get_program_accounts_with_config(
            &clockwork_sdk::ID,
            RpcProgramAccountsConfig {
                filters: Some(vec![RpcFilterType::Memcmp(Memcmp {
                    offset: 0,
                    bytes: MemcmpEncodedBytes::Bytes(Thread::discriminator().to_vec()),
                    encoding: None,
                })]),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    data_slice: None,
                    commitment: Some(CommitmentConfig::finalized()),
                    min_context_slot: None,
                },
                with_context: None,
            },
        )
        .await
        .unwrap()
        .iter()
        .map(|acc| (Thread::try_from(acc.1.data.clone()).unwrap(), acc.1.clone()))
        .collect::<Vec<(Thread, Account)>>();
    accounts[0..10].to_vec()
}

pub async fn get_thread(pubkey: Pubkey) -> Option<Thread> {
    // let client = WasmClient::new("http://74.118.139.8899");
    const HELIUS_API_KEY: &str = dotenv!("HELIUS_API_KEY");
    log::info!("API KEY: {}", HELIUS_API_KEY);
    let url = format!("https://rpc.helius.xyz/?api-key={}", HELIUS_API_KEY);
    let helius_rpc_endpoint = url.as_str();
    let client = WasmClient::new(helius_rpc_endpoint);

    let account = client
        .get_account_with_config(
            &pubkey,
            RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::Base64),
                data_slice: None,
                commitment: Some(CommitmentConfig::finalized()),
                min_context_slot: None,
            },
        )
        .await
        .unwrap()
        .unwrap();

    Some(Thread::try_from(account.data).unwrap())
}

pub async fn simulate_thread(
    thread: Thread,
) -> ClientResult<(Option<TransactionError>, Option<Vec<String>>)> {
    const HELIUS_API_KEY: &str = dotenv!("HELIUS_API_KEY");
    let url = format!("https://rpc.helius.xyz/?api-key={}", HELIUS_API_KEY);
    let helius_rpc_endpoint = url.as_str();
    let client = WasmClient::new(helius_rpc_endpoint);
    let signatory_pubkey =
        Pubkey::from_str("GuJVu6wky7zeVaPkGaasC5vx1eVoiySbEv7UFKZAu837").unwrap();
    let thread_pubkey = Thread::pubkey(thread.authority, thread.id);
    let worker_pubkey = Pubkey::from_str("EvoeDp2WL1TFdLdf9bfJaznsf3YVByisvHM5juYdFBuq").unwrap();

    let first_instruction = if thread.next_instruction.is_some() {
        Instruction {
            program_id: clockwork_sdk::ID,
            accounts: clockwork_thread_program_v1::accounts::ThreadKickoff {
                signatory: signatory_pubkey,
                thread: thread_pubkey,
                worker: worker_pubkey,
            }
            .to_account_metas(Some(false)),
            data: clockwork_thread_program_v1::instruction::ThreadKickoff {}.data(),
        }
    } else {
        Instruction {
            program_id: clockwork_thread_program_v1::ID,
            accounts: clockwork_thread_program_v1::accounts::ThreadExec {
                fee: clockwork_network_program_v1::state::Fee::pubkey(worker_pubkey),
                penalty: clockwork_network_program_v1::state::Penalty::pubkey(worker_pubkey),
                pool: clockwork_network_program_v1::state::Pool::pubkey(0),
                signatory: signatory_pubkey,
                thread: thread_pubkey,
                worker: worker_pubkey,
            }
            .to_account_metas(Some(true)),
            data: clockwork_thread_program_v1::instruction::ThreadExec {}.data(),
        }
    };

    let ixs: Vec<Instruction> = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
        first_instruction,
    ];

    let tx = Transaction::new_with_payer(&ixs, Some(&signatory_pubkey));

    // simulate transaction
    let sim_tx = client.simulate_transaction(tx).await.unwrap();

    Ok((sim_tx.err, sim_tx.logs))
}
