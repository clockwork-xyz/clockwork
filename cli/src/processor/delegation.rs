use anchor_lang::{
    solana_program::{
        instruction::Instruction, system_program, sysvar
    },
    InstructionData, AccountDeserialize, ToAccountMetas
};
use clockwork_network_program::state::{Config, Delegation, Worker};
use spl_associated_token_account::get_associated_token_address;

use crate::{client::Client, errors::CliError};

pub fn create(client: &Client, worker_id: u64) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config = Config::try_deserialize(&mut config_data.as_slice())
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Get worker
    let worker_pubkey = Worker::pubkey(worker_id);
    let worker_data = client
        .get_account_data(&worker_pubkey)
        .map_err(|_err| CliError::AccountNotFound(worker_pubkey.to_string()))?;
    let worker = Worker::try_deserialize(&mut worker_data.as_slice())
        .map_err(|_err| CliError::AccountDataNotParsable(worker_pubkey.to_string()))?;

    // Build ix
    let delegation_pubkey = Delegation::pubkey(worker_pubkey, worker.total_delegations);
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::DelegationCreate {
            associated_token_program: anchor_spl::associated_token::ID,
            authority: client.payer_pubkey(),
            config: Config::pubkey(),
            delegation: delegation_pubkey,
            delegation_tokens: get_associated_token_address(&delegation_pubkey, &config.mint),
            mint: config.mint,
            rent: sysvar::rent::ID,
            system_program: system_program::ID,
            token_program: anchor_spl::token::ID,
            worker: worker_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::DelegationCreate {}.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();

    Ok(())
}

pub fn deposit(
    client: &Client,
    amount: u64,
    delegation_id: u64,
    worker_id: u64,
) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config = Config::try_deserialize(&mut config_data.as_slice())
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // TODO Map the amount using the mint's decimals.

    // Build ix
    let worker_pubkey = Worker::pubkey(worker_id);
    let delegation_pubkey = Delegation::pubkey(worker_pubkey, delegation_id);
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::DelegationDeposit {
            authority: client.payer_pubkey(),
            authority_tokens: get_associated_token_address(&client.payer_pubkey(), &config.mint),
            config: Config::pubkey(),
            delegation: delegation_pubkey,
            delegation_tokens: get_associated_token_address(&delegation_pubkey, &config.mint),
            token_program: anchor_spl::token::ID, 
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::DelegationDeposit { amount }.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();

    Ok(())
}

pub fn withdraw(
    client: &Client,
    amount: u64,
    delegation_id: u64,
    worker_id: u64,
) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config = Config::try_deserialize(&mut config_data.as_slice())
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // TODO Map the amount using the mint's decimals.

    // Build ix
    let worker_pubkey = Worker::pubkey(worker_id);
    let delegation_pubkey = Delegation::pubkey(worker_pubkey, delegation_id);
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::DelegationWithdraw {
            authority: client.payer_pubkey(),
            authority_tokens: get_associated_token_address(&client.payer_pubkey(), &config.mint),
            config: Config::pubkey(),
            delegation: delegation_pubkey,
            delegation_tokens: get_associated_token_address(&delegation_pubkey, &config.mint),
            token_program: anchor_spl::token::ID, 
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::DelegationWithdraw { amount }.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();

    Ok(())
}

pub fn get(client: &Client, delegation_id: u64, worker_id: u64) -> Result<(), CliError> {
    // Get config account
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config = Config::try_deserialize(&mut config_data.as_slice())
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Get the delegation account.
    let worker_pubkey = Worker::pubkey(worker_id);
    let delegation_pubkey = Delegation::pubkey(worker_pubkey, delegation_id);
    let delegation_data = client
        .get_account_data(&delegation_pubkey)
        .map_err(|_err| CliError::AccountNotFound(delegation_pubkey.to_string()))?;
    let delegation = Delegation::try_deserialize(&mut delegation_data.as_slice())
        .map_err(|_err| CliError::AccountDataNotParsable(delegation_pubkey.to_string()))?;

    // Get the delegation's token account.
    let delegation_tokens_pubkey = get_associated_token_address(&delegation_pubkey, &config.mint);
    let token_balance = client
        .get_token_account_balance(&delegation_tokens_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(delegation_pubkey.to_string()))?;

    println!(
        "Address: {}\n{:#?}\nLiquid balance: {}",
        delegation_pubkey, delegation, token_balance.ui_amount_string
    );

    Ok(())
}
