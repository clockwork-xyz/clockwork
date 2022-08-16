use {
    crate::errors::CliError,
    clockwork_client::network::state::{Config, Node, Registry, Snapshot, SnapshotEntry},
    clockwork_client::Client,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    },
};

// pub fn get(client: &Client, address: Pubkey) -> Result<(), CliError> {
//     let node = client
//         .get::<clockwork_client::network::state::Node>(&address)
//         .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
//     println!("{:#?}", node);
//     Ok(())
// }

pub fn register(client: &Client, worker: Keypair) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config_data = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Build ix
    let owner = client.payer().clone();
    let registry_pubkey = Registry::pubkey();
    let registry_data = client
        .get_account_data(&registry_pubkey)
        .map_err(|_err| CliError::AccountNotFound(registry_pubkey.to_string()))?;
    let registry_data = Registry::try_from(registry_data)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    let node_pubkey = Node::pubkey(registry_data.node_count);

    let snapshot_pubkey = Snapshot::pubkey(registry_data.snapshot_count - 1);
    let entry_pubkey = SnapshotEntry::pubkey(snapshot_pubkey, registry_data.node_count);
    let ix = clockwork_client::network::instruction::node_register(
        config_pubkey,
        entry_pubkey,
        config_data.mint,
        node_pubkey,
        registry_pubkey,
        owner.pubkey(),
        snapshot_pubkey,
        worker.pubkey(),
    );
    client.send_and_confirm(&[ix], &[owner, &worker]).unwrap();
    Ok(())
}

pub fn stake(client: &Client, address: Pubkey, amount: u64) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config_data = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Build ix
    let signer = client.payer();
    // let node_pubkey = Node::pubkey(worker);
    let ix = clockwork_client::network::instruction::node_stake(
        amount,
        config_pubkey,
        address,
        config_data.mint,
        signer.pubkey(),
    );

    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}
