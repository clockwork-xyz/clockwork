use anchor_lang::{
    solana_program::{
        instruction::Instruction,
        system_program, sysvar,
    },
    AccountDeserialize, InstructionData, ToAccountMetas
};
use anchor_spl::{associated_token, associated_token::get_associated_token_address, token};
use clockwork_network_program::state::{
    Config, Fee, Penalty, Registry, Snapshot, SnapshotFrame, Worker, WorkerSettings,
};
use solana_sdk::signature::{Keypair, Signer};

use crate::{client::Client, errors::CliError};

pub fn get(client: &Client, id: u64) -> Result<(), CliError> {
    let worker_pubkey = Worker::pubkey(id);
    let worker = client
        .get::<Worker>(&worker_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(worker_pubkey.to_string()))?;

    // Get fee balance
    let fee_pubkey = Fee::pubkey(worker_pubkey);
    let fee_data = client
        .get_account_data(&fee_pubkey)
        .map_err(|_err| CliError::AccountNotFound(fee_pubkey.to_string()))?;
    let fees_min_rent = client
        .get_minimum_balance_for_rent_exemption(fee_data.len())
        .unwrap();
    let fees_balance = client.get_balance(&fee_pubkey).unwrap();
    let fees_total = fees_balance - fees_min_rent;

    // Get penalty balance
    let penalty_pubkey = Penalty::pubkey(worker_pubkey);
    let penalty_data = client
        .get_account_data(&penalty_pubkey)
        .map_err(|_err| CliError::AccountNotFound(penalty_pubkey.to_string()))?;
    let penalty_min_rent = client
        .get_minimum_balance_for_rent_exemption(penalty_data.len())
        .unwrap();
    let penalty_balance = client.get_balance(&penalty_pubkey).unwrap();
    let penalty_total = penalty_balance - penalty_min_rent;

    println!(
        "Address: {}\nFees: {}\nFee account: {}\nPenalty: {}\nPenalty account: {}\n{:#?}",
        worker_pubkey, fees_total, fee_pubkey, penalty_total, penalty_pubkey, worker
    );

    // Get registry
    let registry_pubkey = Registry::pubkey();
    let registry_data = client
        .get_account_data(&registry_pubkey)
        .map_err(|_err| CliError::AccountNotFound(registry_pubkey.to_string()))?;
    let registry = Registry::try_deserialize(&mut registry_data.as_slice())
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    // Get snapshot frame
    let snapshot_pubkey = Snapshot::pubkey(registry.current_epoch);
    let snapshot_frame_pubkey = SnapshotFrame::pubkey(snapshot_pubkey, worker.id);
    match client.get_account_data(&snapshot_frame_pubkey) {
        Err(_err) => {}
        Ok(snapshot_frame_data) => {
            let snapshot_frame = SnapshotFrame::try_deserialize(
                &mut snapshot_frame_data.as_slice(),
            )
            .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;
            println!("{:#?}", snapshot_frame);
        }
    }

    Ok(())
}

pub fn create(client: &Client, signatory: Keypair, silent: bool) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config = Config::try_deserialize(&mut config_data.as_slice())
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Get registry
    let registry_pubkey = Registry::pubkey();
    let registry_data = client
        .get_account_data(&registry_pubkey)
        .map_err(|_err| CliError::AccountNotFound(registry_pubkey.to_string()))?;
    let registry = Registry::try_deserialize(&mut registry_data.as_slice())
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    // Build ix
    let worker_id = registry.total_workers;
    let worker_pubkey = Worker::pubkey(worker_id);
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::WorkerCreate {
            associated_token_program: associated_token::ID,
            authority: client.payer_pubkey(),
            config: Config::pubkey(),
            fee: Fee::pubkey(worker_pubkey),
            penalty: Penalty::pubkey(worker_pubkey),
            mint: config.mint,
            registry: Registry::pubkey(),
            rent: sysvar::rent::ID,
            signatory: signatory.pubkey(),
            system_program: system_program::ID,
            token_program: token::ID,
            worker: worker_pubkey,
            worker_tokens: get_associated_token_address(&worker_pubkey, &config.mint),
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::WorkerCreate {}.data(),
    };
    client
        .send_and_confirm(&[ix], &[client.payer(), &signatory])
        .unwrap();
    if !silent {
        get(client, worker_id)?;
    }
    Ok(())
}

pub fn update(client: &Client, id: u64, signatory: Option<Keypair>) -> Result<(), CliError> {
    // Derive worker keypair.
    let worker_pubkey = Worker::pubkey(id);
    let worker = client
        .get::<Worker>(&worker_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(worker_pubkey.to_string()))?;

    // Build and submit tx.
    let settings = WorkerSettings {
        commission_rate: 0,
        signatory: signatory.map_or(worker.signatory, |v| v.pubkey()),
    };
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::WorkerUpdate {
            authority: client.payer_pubkey(),
            system_program: system_program::ID,
            worker: worker_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::WorkerUpdate { settings }.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, worker.id)?;
    Ok(())
}
