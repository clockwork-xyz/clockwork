use std::sync::Arc;

use clockwork_client::{automation::state::Trigger, network::state::Worker};
use clockwork_utils::automation::PAYER_PUBKEY;
use log::info;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig},
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction, signature::Keypair, signer::Signer,
    transaction::Transaction,
};

use crate::versioned_automation::VersionedAutomation;

/// Max byte size of a serialized transaction.
static TRANSACTION_MESSAGE_SIZE_LIMIT: usize = 1_232;

/// Max compute units that may be used by transaction.
static TRANSACTION_COMPUTE_UNIT_LIMIT: u32 = 1_400_000;

/// The buffer amount to add to transactions' compute units in case on-chain PDA derivations take more CUs than used in simulation.
static TRANSACTION_COMPUTE_UNIT_BUFFER: u32 = 1000;

pub async fn build_automation_exec_tx(
    client: Arc<RpcClient>,
    payer: &Keypair,
    automation: VersionedAutomation,
    automation_pubkey: Pubkey,
    worker_id: u64,
) -> Option<Transaction> {
    // Grab the automation and relevant data.
    let now = std::time::Instant::now();
    let blockhash = client.get_latest_blockhash().await.unwrap();
    let signatory_pubkey = payer.pubkey();

    // Build the first instruction of the transaction.
    let first_instruction = if automation.next_instruction().is_some() {
        build_exec_ix(automation, automation_pubkey, signatory_pubkey, worker_id)
    } else {
        build_kickoff_ix(automation, automation_pubkey, signatory_pubkey, worker_id)
    };

    // Simulate the transactino and pack as many instructions as possible until we hit mem/cpu limits.
    // TODO Migrate to versioned transactions.
    let mut ixs: Vec<Instruction> = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(TRANSACTION_COMPUTE_UNIT_LIMIT),
        first_instruction,
    ];
    let mut successful_ixs: Vec<Instruction> = vec![];
    let mut units_consumed: Option<u64> = None;
    loop {
        let mut sim_tx = Transaction::new_with_payer(&ixs, Some(&signatory_pubkey));
        sim_tx.sign(&[payer], blockhash);

        // Exit early if the transaction exceeds the size limit.
        if sim_tx.message_data().len() > TRANSACTION_MESSAGE_SIZE_LIMIT {
            break;
        }

        // Run the simulation.
        match client
            .simulate_transaction_with_config(
                &sim_tx,
                RpcSimulateTransactionConfig {
                    replace_recent_blockhash: true,
                    commitment: Some(CommitmentConfig::processed()),
                    accounts: Some(RpcSimulateTransactionAccountsConfig {
                        encoding: Some(UiAccountEncoding::Base64Zstd),
                        addresses: vec![automation_pubkey.to_string()],
                    }),
                    ..RpcSimulateTransactionConfig::default()
                },
            )
            .await
        {
            // If there was a simulation error, stop packing and exit now.
            Err(_err) => {
                break;
            }

            // If the simulation was successful, pack the ix into the tx.
            Ok(response) => {
                if response.value.err.is_some() {
                    if successful_ixs.is_empty() {
                        info!(
                            "automation: {} simulation_error: \"{}\" logs: {:?}",
                            automation_pubkey,
                            response.value.err.unwrap(),
                            response.value.logs.unwrap_or(vec![])
                        );
                    }
                    break;
                }

                // Update flag tracking if at least one instruction succeed.
                successful_ixs = ixs.clone();

                // Record the compute units consumed by the simulation.
                if response.value.units_consumed.is_some() {
                    units_consumed = response.value.units_consumed;
                }

                // Parse the resulting automation account for the next instruction to simulate.
                if let Some(ui_accounts) = response.value.accounts {
                    if let Some(Some(ui_account)) = ui_accounts.get(0) {
                        if let Some(account) = ui_account.decode::<Account>() {
                            if let Ok(sim_automation) = VersionedAutomation::try_from(account.data)
                            {
                                if sim_automation.next_instruction().is_some() {
                                    if let Some(exec_context) = sim_automation.exec_context() {
                                        if exec_context
                                            .execs_since_slot
                                            .lt(&sim_automation.rate_limit())
                                        {
                                            ixs.push(build_exec_ix(
                                                sim_automation,
                                                automation_pubkey,
                                                signatory_pubkey,
                                                worker_id,
                                            ));
                                        } else {
                                            // Exit early if the automation has reached its rate limit.
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
        let units_committed = std::cmp::min(
            (units_consumed as u32) + TRANSACTION_COMPUTE_UNIT_BUFFER,
            TRANSACTION_COMPUTE_UNIT_LIMIT,
        );
        _ = std::mem::replace(
            &mut successful_ixs[0],
            ComputeBudgetInstruction::set_compute_unit_limit(units_committed),
        );
    }

    // Build and return the signed transaction.
    let mut tx = Transaction::new_with_payer(&successful_ixs, Some(&signatory_pubkey));
    tx.sign(&[payer], blockhash);
    info!(
        "automation: {:?} sim_duration: {:?} instruction_count: {:?} compute_units: {:?} tx_sig: {:?}",
        automation_pubkey,
        now.elapsed(),
        successful_ixs.len(),
        units_consumed,
        tx.signatures[0]
    );
    Some(tx)
}

fn build_kickoff_ix(
    automation: VersionedAutomation,
    automation_pubkey: Pubkey,
    signatory_pubkey: Pubkey,
    worker_id: u64,
) -> Instruction {
    // Build the instruction.
    let mut kickoff_ix = clockwork_client::automation::instruction::automation_kickoff(
        signatory_pubkey,
        automation_pubkey,
        Worker::pubkey(worker_id),
    );

    // If the automation's trigger is account-based, inject the triggering account.
    match automation.trigger() {
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
    automation: VersionedAutomation,
    automation_pubkey: Pubkey,
    signatory_pubkey: Pubkey,
    worker_id: u64,
) -> Instruction {
    // Build the instruction.
    let mut exec_ix = clockwork_client::automation::instruction::automation_exec(
        signatory_pubkey,
        automation_pubkey,
        Worker::pubkey(worker_id),
    );

    if let Some(next_instruction) = automation.next_instruction() {
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
