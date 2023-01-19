use spl_associated_token_account::get_associated_token_address;

use {
    crate::errors::CliError,
    clockwork_client::network::state::{Config, Delegation, Worker},
    clockwork_client::Client,
};

pub fn create(client: &Client, worker_id: u64) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Get worker
    let worker_pubkey = Worker::pubkey(worker_id);
    let worker_data = client
        .get_account_data(&worker_pubkey)
        .map_err(|_err| CliError::AccountNotFound(worker_pubkey.to_string()))?;
    let worker = Worker::try_from(worker_data)
        .map_err(|_err| CliError::AccountDataNotParsable(worker_pubkey.to_string()))?;

    // Build ix
    let delegation_pubkey = Delegation::pubkey(worker_pubkey, worker.total_delegations);
    let ix = clockwork_client::network::instruction::delegation_create(
        client.payer_pubkey(),
        delegation_pubkey,
        config.mint,
        worker_pubkey,
    );
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
    let config = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // TODO Map the amount using the mint's decimals.

    // Build ix
    let worker_pubkey = Worker::pubkey(worker_id);
    let delegation_pubkey = Delegation::pubkey(worker_pubkey, delegation_id);
    let ix = clockwork_client::network::instruction::delegation_deposit(
        amount,
        client.payer_pubkey(),
        delegation_pubkey,
        config.mint,
    );
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
    let config = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // TODO Map the amount using the mint's decimals.

    // Build ix
    let worker_pubkey = Worker::pubkey(worker_id);
    let delegation_pubkey = Delegation::pubkey(worker_pubkey, delegation_id);
    let ix = clockwork_client::network::instruction::delegation_withdraw(
        amount,
        client.payer_pubkey(),
        delegation_pubkey,
        config.mint,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();

    Ok(())
}

pub fn get(client: &Client, delegation_id: u64, worker_id: u64) -> Result<(), CliError> {
    // Get config account
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Get the delegation account.
    let worker_pubkey = Worker::pubkey(worker_id);
    let delegation_pubkey = Delegation::pubkey(worker_pubkey, delegation_id);
    let delegation_data = client
        .get_account_data(&delegation_pubkey)
        .map_err(|_err| CliError::AccountNotFound(delegation_pubkey.to_string()))?;
    let delegation = Delegation::try_from(delegation_data)
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
