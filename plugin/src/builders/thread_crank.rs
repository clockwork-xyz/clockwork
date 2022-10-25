use {
    clockwork_client::{
        network::objects::Worker,
        thread::objects::{Thread, Trigger, TriggerContext},
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
    std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
        sync::Arc,
    },
};

static TRANSACTION_SIZE_LIMIT: usize = 1_232; // Max byte size of a serialized transaction

pub async fn build_crank_txs(
    client: Arc<ClockworkClient>,
    crankable_threads: DashSet<Pubkey>,
    worker_id: u64,
) -> Vec<Transaction> {
    // Build the set of crank transactions
    // TODO Use rayon to parallelize this operation
    let txs = crankable_threads
        .iter()
        .filter_map(|thread_pubkey_ref| {
            build_crank_tx(client.clone(), *thread_pubkey_ref.key(), worker_id)
        })
        .collect::<Vec<Transaction>>();
    txs
}

fn build_crank_tx(
    client: Arc<ClockworkClient>,
    thread_pubkey: Pubkey,
    worker_id: u64,
) -> Option<Transaction> {
    // Build the first crank ix
    let thread = match client.get::<Thread>(&thread_pubkey) {
        Err(_err) => return None,
        Ok(thread) => thread,
    };
    let blockhash = client.get_latest_blockhash().unwrap();
    let signatory_pubkey = client.payer_pubkey();

    // Pre-simulate crank ixs and pack into tx
    let first_instruction = if thread.next_instruction.is_some() {
        build_crank_ix(client.clone(), thread, signatory_pubkey, worker_id)
    } else {
        build_kickoff_ix(client.clone(), thread, signatory_pubkey, worker_id)
    };
    let mut ixs: Vec<Instruction> = vec![first_instruction];

    // Pre-simulate crank ixs and pack as many as possible into tx.
    let mut tx: Transaction = Transaction::new_with_payer(&vec![], Some(&signatory_pubkey));
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
            // If there was an error, stop packing and continue with the cranks up until this one.
            Err(_err) => {
                break;
            }

            // If the simulation was successful, pack the crank ix into the tx.
            Ok(response) => {
                // If there was an error, then stop packing.
                if response.value.err.is_some() {
                    info!(
                        "Error simulating tx: {} logs: {:#?}",
                        response.value.err.unwrap(),
                        response.value.logs
                    );
                    break;
                }

                // Save the simulated tx. It is okay to submit.
                tx = sim_tx;

                // Parse the resulting thread account for the next crank ix to simulate.
                if let Some(ui_accounts) = response.value.accounts {
                    if let Some(Some(ui_account)) = ui_accounts.get(0) {
                        if let Some(account) = ui_account.decode::<Account>() {
                            if let Ok(sim_thread) = Thread::try_from(account.data) {
                                if sim_thread.next_instruction.is_some() {
                                    ixs.push(build_crank_ix(
                                        client.clone(),
                                        sim_thread,
                                        signatory_pubkey,
                                        worker_id,
                                    ));
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
        "Time spent packing {} cranks: {:#?}",
        tx.message.instructions.len(),
        now.elapsed()
    );

    if tx.message.instructions.len() == 0 {
        return None;
    }

    Some(tx)
}

fn build_kickoff_ix(
    client: Arc<ClockworkClient>,
    thread: Thread,
    signatory_pubkey: Pubkey,
    worker_id: u64,
) -> Instruction {
    // If this thread is an account listener, grab the account and create the data_hash.
    let mut trigger_account_pubkey: Option<Pubkey> = None;
    let mut data_hash: Option<u64> = None;
    match thread.trigger {
        Trigger::Account {
            address,
            offset,
            size,
        } => {
            // Save the trigger account.
            trigger_account_pubkey = Some(address);

            // Begin computing the data hash of this account.
            let data = client.get_account_data(&address).unwrap();
            let mut hasher = DefaultHasher::new();
            if offset + size < data.len() {
                data[offset..(offset + size)].hash(&mut hasher);
            }

            // Check the exec context for the prior data hash.
            match thread.exec_context.clone() {
                None => {
                    // This thread has not begun executing yet.
                    // There is no prior data hash to include in our hash.
                    data_hash = Some(hasher.finish());
                }
                Some(exec_context) => {
                    match exec_context.trigger_context {
                        TriggerContext::Account {
                            data_hash: prior_data_hash,
                        } => {
                            // Inject the prior data hash as a seed.
                            prior_data_hash.hash(&mut hasher);
                            data_hash = Some(hasher.finish());
                        }
                        _ => {}
                    }
                }
            };
        }
        _ => {}
    }

    // Build the instruction.
    let thread_pubkey = Thread::pubkey(thread.authority, thread.id);
    let mut kickoff_ix = clockwork_client::thread::instruction::thread_kickoff(
        data_hash,
        thread_pubkey,
        signatory_pubkey,
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

fn build_crank_ix(
    _client: Arc<ClockworkClient>,
    thread: Thread,
    signatory_pubkey: Pubkey,
    worker_id: u64,
) -> Instruction {
    // Build the instruction.
    let thread_pubkey = Thread::pubkey(thread.authority, thread.id);
    let mut crank_ix = clockwork_client::thread::instruction::thread_crank(
        thread_pubkey,
        signatory_pubkey,
        Worker::pubkey(worker_id),
    );

    if let Some(next_instruction) = thread.next_instruction {
        // Inject the target program account to the ix.
        crank_ix.accounts.push(AccountMeta::new_readonly(
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
            crank_ix.accounts.push(match acc.is_writable {
                true => AccountMeta::new(acc_pubkey, false),
                false => AccountMeta::new_readonly(acc_pubkey, false),
            })
        }
    }

    crank_ix
}
