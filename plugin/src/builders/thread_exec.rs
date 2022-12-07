use solana_sdk::compute_budget::ComputeBudgetInstruction;

use {
    clockwork_client::{
        network::state::Worker,
        thread::state::{Thread, Trigger},
        Client as ClockworkClient,
    },
    dashmap::DashSet,
    log::info,
    solana_account_decoder::UiAccountEncoding,
    solana_client::rpc_config::{
        RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig,
    },
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{account::Account, commitment_config::CommitmentConfig, transaction::Transaction},
    std::sync::Arc,
};

/// Max byte size of a serialized transaction.
static TRANSACTION_SIZE_LIMIT: usize = 1_232;

/// Max compute units that may be used by transaction.
static COMPUTE_UNIT_LIMIT: u32 = 1_400_000;

pub async fn build_thread_exec_txs(
    client: Arc<ClockworkClient>,
    executable_threads: DashSet<Pubkey>,
    worker_id: u64,
) -> Vec<Transaction> {
    // Build the set of exec transactions
    // TODO Use rayon to parallelize this operation
    let txs = executable_threads
        .iter()
        .filter_map(|thread_pubkey_ref| {
            build_thread_exec_tx(client.clone(), *thread_pubkey_ref.key(), worker_id)
        })
        .collect::<Vec<Transaction>>();
    txs
}

fn build_thread_exec_tx(
    client: Arc<ClockworkClient>,
    thread_pubkey: Pubkey,
    worker_id: u64,
) -> Option<Transaction> {
    // Build the first ix
    let thread = match client.get::<Thread>(&thread_pubkey) {
        Err(_err) => return None,
        Ok(thread) => thread,
    };
    let blockhash = client.get_latest_blockhash().unwrap();
    let signatory_pubkey = client.payer_pubkey();

    // Get the first instruction to pack into the tx.
    let first_instruction = if thread.next_instruction.is_some() {
        build_exec_ix(thread, signatory_pubkey, worker_id)
    } else {
        build_kickoff_ix(thread, signatory_pubkey, worker_id)
    };

    // Pre-simulate exec ixs and pack as many as possible into tx.
    let compute_unit_ix = ComputeBudgetInstruction::set_compute_unit_limit(COMPUTE_UNIT_LIMIT);
    let mut units_consumed: Option<u64> = None;
    let mut ixs: Vec<Instruction> = vec![compute_unit_ix.clone(), first_instruction];
    let mut did_simulation_succeed: bool = false;
    let now = std::time::Instant::now();
    loop {
        let mut sim_tx = Transaction::new_with_payer(&ixs, Some(&signatory_pubkey));
        sim_tx.sign(&[client.payer()], blockhash);

        // Exit early if tx exceeds Solana's size limit.
        // TODO With QUIC and Transaction v2 lookup tables, Solana will soon support much larger transaction sizes.
        if sim_tx.message_data().len() > TRANSACTION_SIZE_LIMIT {
            break;
        }

        // Simulate the complete packed tx.
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
            // If there was an error, stop packing and continue with the ixs up until this one.
            Err(_err) => {
                break;
            }

            // If the simulation was successful, pack the ix into the tx.
            Ok(response) => {
                // If there was an error, then stop packing.
                if response.value.err.is_some() {
                    info!(
                        "Error simulating thread: {} tx: {} logs: {:#?}",
                        thread_pubkey,
                        response.value.err.unwrap(),
                        response.value.logs
                    );
                    break;
                }

                // Save the simulated tx. It is okay to submit.
                did_simulation_succeed = true;

                // Save the consumed compute units.
                if response.value.units_consumed.is_some() {
                    units_consumed = response.value.units_consumed;
                }

                // Parse the resulting thread account for the next ix to simulate.
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

    info!(
        "sim_duration: {:#?} instruction_count: {:#?} compute_units: {:#?}",
        now.elapsed(),
        ixs.len(),
        units_consumed
    );

    // If the simulation never succeeded, exit early. There is nothing to do.
    if !did_simulation_succeed {
        return None;
    }

    // Set the compute unit limit to be slightly above what was used in the simulation.
    if let Some(units_consumed) = units_consumed {
        // TODO Is this buffer needed? It is intended to account for variations in PDA derivation cost.
        let compute_unit_buffer = 1_000;
        _ = std::mem::replace(
            &mut ixs[0],
            ComputeBudgetInstruction::set_compute_unit_limit(
                (units_consumed + compute_unit_buffer) as u32,
            ),
        );
    }

    // Build and return the signed tx.
    let mut tx = Transaction::new_with_payer(&ixs, Some(&signatory_pubkey));
    tx.sign(&[client.payer()], blockhash);
    Some(tx)
}

fn build_kickoff_ix(thread: Thread, signatory_pubkey: Pubkey, worker_id: u64) -> Instruction {
    // If this thread is an account listener, grab the account and create the data_hash.
    let mut trigger_account_pubkey: Option<Pubkey> = None;
    match thread.trigger {
        Trigger::Account {
            address,
            offset: _,
            size: _,
        } => {
            // Save the trigger account.
            trigger_account_pubkey = Some(address);
        }
        _ => {}
    }

    // Build the instruction.
    let thread_pubkey = Thread::pubkey(thread.authority, thread.id);
    let mut kickoff_ix = clockwork_client::thread::instruction::thread_kickoff(
        signatory_pubkey,
        thread_pubkey,
        Worker::pubkey(worker_id),
    );

    // Inject the trigger account.
    match trigger_account_pubkey {
        None => {}
        Some(pubkey) => kickoff_ix.accounts.push(AccountMeta {
            pubkey,
            is_signer: false,
            is_writable: false,
        }),
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
        // Inject the target program account to the ix.
        exec_ix.accounts.push(AccountMeta::new_readonly(
            next_instruction.program_id,
            false,
        ));

        // Inject the worker pubkey as the Clockwork "payer" account
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
