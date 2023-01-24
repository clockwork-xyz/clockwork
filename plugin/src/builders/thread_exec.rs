use std::sync::Arc;

use clockwork_client::{
    network::state::Worker,
    thread::state::{Thread, Trigger},
    Client as ClockworkClient,
};
use log::info;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{
    RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction, transaction::Transaction,
};

/// Max byte size of a serialized transaction.
static TRANSACTION_MESSAGE_SIZE_LIMIT: usize = 1_232;

/// Max compute units that may be used by transaction.
static TRANSACTION_COMPUTE_UNIT_LIMIT: u32 = 1_400_000;


pub fn build_thread_exec_tx(
    client: Arc<ClockworkClient>,
    thread_pubkey: Pubkey,
    worker_id: u64,
) -> Option<Transaction> {
    // Grab the thread and relevant data.
    let thread = match client.get::<Thread>(&thread_pubkey) {
        Err(_err) => return None,
        Ok(thread) => thread,
    };
    let blockhash = client.get_latest_blockhash().unwrap();
    let signatory_pubkey = client.payer_pubkey();

    // Build the first instruction of the transaction.
    let first_instruction = if thread.next_instruction.is_some() {
        build_exec_ix(thread, signatory_pubkey, worker_id)
    } else {
        build_kickoff_ix(thread, signatory_pubkey, worker_id)
    };

    // Simulate the transactino and pack as many instructions as possible until we hit mem/cpu limits.
    // TODO Migrate to versioned transactions.
    let mut ixs: Vec<Instruction> = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(TRANSACTION_COMPUTE_UNIT_LIMIT),
        first_instruction,
    ];
    let mut successful_ixs: Vec<Instruction> = vec![];
    let mut units_consumed: Option<u64> = None;
    let now = std::time::Instant::now();
    loop {
        let mut sim_tx = Transaction::new_with_payer(&ixs, Some(&signatory_pubkey));
        sim_tx.sign(&[client.payer()], blockhash);

        // Exit early if the transaction exceeds the size limit.
        if sim_tx.message_data().len() > TRANSACTION_MESSAGE_SIZE_LIMIT {
            break;
        }

        // Run the simulation.
        match client.simulate_transaction_with_config(
            &sim_tx,
            RpcSimulateTransactionConfig {
                replace_recent_blockhash: true,
                commitment: Some(CommitmentConfig::processed()),
                accounts: Some(RpcSimulateTransactionAccountsConfig {
                    encoding: Some(UiAccountEncoding::Base64Zstd),
                    addresses: vec![thread_pubkey.to_string()],
                }),
                ..RpcSimulateTransactionConfig::default()
            },
        ) {
            // If there was a simulation error, stop packing and exit now.
            Err(_err) => {
                break;
            }

            // If the simulation was successful, pack the ix into the tx.
            Ok(response) => {
                if response.value.err.is_some() {
                    info!(
                        "Error simulating thread: {} error: \"{}\" logs: {:?}",
                        thread_pubkey,
                        response.value.err.unwrap(),
                        response.value.logs.unwrap_or(vec![])
                    );
                    break;
                }

                // Update flag tracking if at least one instruction succeed.
                successful_ixs = ixs.clone();

                // Record the compute units consumed by the simulation.
                if response.value.units_consumed.is_some() {
                    units_consumed = response.value.units_consumed;
                }

                // Parse the resulting thread account for the next instruction to simulate.
                if let Some(ui_accounts) = response.value.accounts {
                    if let Some(Some(ui_account)) = ui_accounts.get(0) {
                        if let Some(account) = ui_account.decode::<Account>() {
                            if let Ok(sim_thread) = Thread::try_from(account.data) {
                                if sim_thread.next_instruction.is_some() {
                                    if let Some(exec_context) = sim_thread.exec_context {
                                        if exec_context.execs_since_slot.lt(&sim_thread.rate_limit)
                                        {
                                            ixs.push(build_exec_ix(
                                                sim_thread,
                                                signatory_pubkey,
                                                worker_id,
                                            ));
                                        } else {
                                            // Exit early if the thread has reached its rate limit.
                                            break;
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // If there were no successful instructions, then exit early. There is nothing to do.
    if successful_ixs.is_empty() {
        return None;
    }

    // Set the transaction's compute unit limit to be exactly the amount that was used in simulation.
    if let Some(units_consumed) = units_consumed {
        _ = std::mem::replace(
            &mut successful_ixs[0],
            ComputeBudgetInstruction::set_compute_unit_limit(units_consumed as u32),
        );
    }

    // Build and return the signed transaction.
    let mut tx = Transaction::new_with_payer(&successful_ixs, Some(&signatory_pubkey));
    tx.sign(&[client.payer()], blockhash);
    info!(
        "sim_duration: {:#?} instruction_count: {:#?} compute_units: {:#?} tx_sig: {:#?}",
        now.elapsed(),
        successful_ixs.len(),
        units_consumed,
        tx.signatures[0]
    );
    Some(tx)
}

fn build_kickoff_ix(thread: Thread, signatory_pubkey: Pubkey, worker_id: u64) -> Instruction {
    // Build the instruction.
    let thread_pubkey = Thread::pubkey(thread.authority, thread.id);
    let mut kickoff_ix = clockwork_client::thread::instruction::thread_kickoff(
        signatory_pubkey,
        thread_pubkey,
        Worker::pubkey(worker_id),
    );

    // If the thread's trigger is account-based, inject the triggering account.
    match thread.trigger {
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

fn build_exec_ix(thread: Thread, signatory_pubkey: Pubkey, worker_id: u64) -> Instruction {
    // Build the instruction.
    let thread_pubkey = Thread::pubkey(thread.authority, thread.id);
    let mut exec_ix = clockwork_client::thread::instruction::thread_exec(
        signatory_pubkey,
        thread_pubkey,
        Worker::pubkey(worker_id),
    );

    if let Some(next_instruction) = thread.next_instruction {
        // Inject the target program account.
        exec_ix.accounts.push(AccountMeta::new_readonly(
            next_instruction.program_id,
            false,
        ));

        // Inject the worker pubkey as the dynamic "payer" account.
        for acc in next_instruction.clone().accounts {
            let acc_pubkey = if acc.pubkey == clockwork_utils::PAYER_PUBKEY {
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
