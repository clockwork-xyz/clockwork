use {
    crate::errors::CliError,
    clockwork_client::network::objects::{Config, Registry, Worker},
    clockwork_client::Client,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    },
};

pub fn create(client: &Client, signatory: Keypair) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Get registry
    let registry_pubkey = Registry::pubkey();
    let registry_data = client
        .get_account_data(&registry_pubkey)
        .map_err(|_err| CliError::AccountNotFound(registry_pubkey.to_string()))?;
    let registry_data = Registry::try_from(registry_data)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    // Build ix
    let worker_pubkey = Worker::pubkey(registry_data.total_workers);
    let ix = clockwork_client::network::instruction::worker_create(
        client.payer_pubkey(),
        config.mint,
        signatory.pubkey(),
        worker_pubkey,
    );
    client
        .send_and_confirm(&[ix], &[client.payer(), &signatory])
        .unwrap();

    Ok(())
}

pub fn delegate_stake(client: &Client, amount: u64, worker_pubkey: Pubkey) -> Result<(), CliError> {
    // // Get config data
    // let config_pubkey = Config::pubkey();
    // let config_data = client
    //     .get_account_data(&config_pubkey)
    //     .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    // let config_data = Config::try_from(config_data)
    //     .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // // Build ix
    // let signer = client.payer();
    // // let node_pubkey = Node::pubkey(worker);
    // let ix = clockwork_client::network::instruction::node_stake(
    //     amount,
    //     config_pubkey,
    //     address,
    //     config_data.mint,
    //     signer.pubkey(),
    // );

    // client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}
