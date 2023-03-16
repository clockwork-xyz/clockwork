use anchor_lang::{
    prelude::{AccountMeta, Pubkey},
    solana_program::instruction::Instruction,
    Discriminator, InstructionData, ToAccountMetas,
};
use clockwork_sdk::{state::Thread, utils::PAYER_PUBKEY};
use clockwork_thread_program_v2::state::{Trigger, VersionedThread};
use dotenv_codegen::dotenv;
use solana_client_wasm::{
    solana_sdk::{
        account::Account,
        commitment_config::CommitmentConfig,
        compute_budget::ComputeBudgetInstruction,
        transaction::{Transaction, TransactionError},
    },
    utils::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
    },
    ClientResult, WasmClient,
};
use solana_extra_wasm::account_decoder::UiAccountEncoding;
use std::str::FromStr;

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

pub async fn get_thread(pubkey: Pubkey) -> VersionedThread {
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

    VersionedThread::try_from(account.data).unwrap()
}

pub async fn simulate_thread(
    thread: VersionedThread,
    thread_pubkey: Pubkey,
) -> ClientResult<(Option<TransactionError>, Option<Vec<String>>)> {
    const HELIUS_API_KEY: &str = dotenv!("HELIUS_API_KEY");
    let url = format!("https://rpc.helius.xyz/?api-key={}", HELIUS_API_KEY);
    let helius_rpc_endpoint = url.as_str();
    let client = WasmClient::new(helius_rpc_endpoint);
    let signatory_pubkey =
        Pubkey::from_str("GuJVu6wky7zeVaPkGaasC5vx1eVoiySbEv7UFKZAu837").unwrap();
    let worker_pubkey = Pubkey::from_str("EvoeDp2WL1TFdLdf9bfJaznsf3YVByisvHM5juYdFBuq").unwrap();

    let first_instruction = if thread.next_instruction().is_some() {
        build_exec_ix(
            thread.clone(),
            thread_pubkey,
            signatory_pubkey,
            worker_pubkey,
        )
    } else {
        build_kickoff_ix(
            thread.clone(),
            thread_pubkey,
            signatory_pubkey,
            worker_pubkey,
        )
    };

    let ixs: Vec<Instruction> = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
        first_instruction,
    ];

    let tx = Transaction::new_with_payer(&ixs, Some(&signatory_pubkey));

    // simulate transaction
    let sim_tx = client.simulate_transaction(&tx).await.unwrap();
    Ok((sim_tx.err, sim_tx.logs))
}

fn build_kickoff_ix(
    thread: VersionedThread,
    thread_pubkey: Pubkey,
    signatory_pubkey: Pubkey,
    worker_pubkey: Pubkey,
) -> Instruction {
    // Build the instruction.
    let mut kickoff_ix = match thread {
        VersionedThread::V1(_) => Instruction {
            program_id: thread.program_id(),
            accounts: clockwork_thread_program_v1::accounts::ThreadKickoff {
                signatory: signatory_pubkey,
                thread: thread_pubkey,
                worker: worker_pubkey,
            }
            .to_account_metas(Some(false)),
            data: clockwork_thread_program_v1::instruction::ThreadKickoff {}.data(),
        },
        VersionedThread::V2(_) => Instruction {
            program_id: thread.program_id(),
            accounts: clockwork_thread_program_v2::accounts::ThreadKickoff {
                signatory: signatory_pubkey,
                thread: thread_pubkey,
                worker: worker_pubkey,
            }
            .to_account_metas(Some(false)),
            data: clockwork_thread_program_v2::instruction::ThreadKickoff {}.data(),
        },
    };

    // If the thread's trigger is account-based, inject the triggering account.
    match thread.trigger() {
        Trigger::Account {
            address,
            offset: _,
            size: _,
        } => kickoff_ix.accounts.push(AccountMeta {
            pubkey: address,
            is_signer: false,
            is_writable: false,
        }),
        _ => {}
    }

    kickoff_ix
}

fn build_exec_ix(
    thread: VersionedThread,
    thread_pubkey: Pubkey,
    signatory_pubkey: Pubkey,
    worker_pubkey: Pubkey,
) -> Instruction {
    // Build the instruction.
    let mut exec_ix = match thread {
        VersionedThread::V1(_) => Instruction {
            program_id: thread.program_id(),
            accounts: clockwork_thread_program_v1::accounts::ThreadExec {
                fee: clockwork_network_program::state::Fee::pubkey(worker_pubkey),
                penalty: clockwork_network_program::state::Penalty::pubkey(worker_pubkey),
                pool: clockwork_network_program::state::Pool::pubkey(0),
                signatory: signatory_pubkey,
                thread: thread_pubkey,
                worker: worker_pubkey,
            }
            .to_account_metas(Some(true)),
            data: clockwork_thread_program_v1::instruction::ThreadExec {}.data(),
        },
        VersionedThread::V2(_) => Instruction {
            program_id: thread.program_id(),
            accounts: clockwork_thread_program_v2::accounts::ThreadExec {
                fee: clockwork_network_program::state::Fee::pubkey(worker_pubkey),
                pool: clockwork_network_program::state::Pool::pubkey(0),
                signatory: signatory_pubkey,
                thread: thread_pubkey,
                worker: worker_pubkey,
            }
            .to_account_metas(Some(true)),
            data: clockwork_thread_program_v2::instruction::ThreadExec {}.data(),
        },
    };

    if let Some(next_instruction) = thread.next_instruction() {
        // Inject the target program account.
        exec_ix.accounts.push(AccountMeta::new_readonly(
            next_instruction.program_id,
            false,
        ));

        // Inject the worker pubkey as the dynamic "payer" account.
        for acc in next_instruction.clone().accounts {
            let acc_pubkey = if acc.pubkey == PAYER_PUBKEY {
                signatory_pubkey
            } else {
                acc.pubkey
            };
            exec_ix.accounts.push(match acc.is_writable {
                true => AccountMeta::new(acc_pubkey, false),
                false => AccountMeta::new_readonly(acc_pubkey, false),
            })
        }
    }

    exec_ix
}
