use {
    crate::errors::CliError,
    clockwork_client::network::objects::{Config, Registry, Worker},
    clockwork_client::Client,
    solana_sdk::signature::{Keypair, Signer},
};

pub fn get(client: &Client, id: u64) -> Result<(), CliError> {
    let worker_pubkey = Worker::pubkey(id);
    let worker = client
        .get::<Worker>(&worker_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(worker_pubkey.to_string()))?;
    println!("{:#?}", worker);
    Ok(())
}

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
    let registry = Registry::try_from(registry_data)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    // Build ix
    let worker_id = registry.total_workers;
    let worker_pubkey = Worker::pubkey(worker_id);
    let ix = clockwork_client::network::instruction::worker_create(
        client.payer_pubkey(),
        config.mint,
        signatory.pubkey(),
        worker_pubkey,
    );
    client
        .send_and_confirm(&[ix], &[client.payer(), &signatory])
        .unwrap();
    get(client, worker_id)?;
    Ok(())
}
