use {
    crate::errors::CliError,
    cronos_client::network::state::{Authority, Config, Node, Registry, Snapshot, SnapshotEntry},
    cronos_client::Client,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    },
};

pub fn get(client: &Client, address: Pubkey) -> Result<(), CliError> {
    let node = client
        .get::<cronos_client::network::state::Node>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", node);
    Ok(())
}

pub fn get_by_worker(client: &Client, worker: Pubkey) -> Result<(), CliError> {
    let node_pubkey = Node::pubkey(worker);
    get(client, node_pubkey)
}

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
    let authority_pubkey = Authority::pubkey();
    let node_pubkey = Node::pubkey(worker.pubkey());
    let registry_pubkey = Registry::pubkey();
    let registry_data = client
        .get_account_data(&registry_pubkey)
        .map_err(|_err| CliError::AccountNotFound(registry_pubkey.to_string()))?;
    let registry_data = Registry::try_from(registry_data)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    let snapshot_pubkey = Snapshot::pubkey(registry_data.snapshot_count - 1);
    let entry_pubkey = SnapshotEntry::pubkey(snapshot_pubkey, registry_data.node_count);
    let snapshot_queue_pubkey =
        cronos_client::scheduler::state::Queue::pubkey(authority_pubkey, "snapshot".into());
    let snapshot_task_pubkey = cronos_client::scheduler::state::Task::pubkey(
        snapshot_queue_pubkey,
        (registry_data.node_count + 1).into(),
    );

    let cleanup_queue_pubkey =
        cronos_client::scheduler::state::Queue::pubkey(authority_pubkey, "cleanup".into());
    let cleanup_task_pubkey = cronos_client::scheduler::state::Task::pubkey(
        cleanup_queue_pubkey,
        (registry_data.node_count + 1).into(),
    );

    let ix = cronos_client::network::instruction::node_register(
        authority_pubkey,
        cleanup_queue_pubkey,
        cleanup_task_pubkey,
        config_pubkey,
        entry_pubkey,
        config_data.mint,
        node_pubkey,
        owner.pubkey(),
        registry_pubkey,
        snapshot_pubkey,
        snapshot_queue_pubkey,
        snapshot_task_pubkey,
        worker.pubkey(),
    );
    client.send_and_confirm(&[ix], &[owner, &worker]).unwrap();
    get(client, node_pubkey)
}

pub fn stake(client: &Client, amount: u64, worker: Pubkey) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config_data = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Build ix
    let signer = client.payer();
    let node_pubkey = Node::pubkey(worker);
    let ix = cronos_client::network::instruction::node_stake(
        amount,
        config_pubkey,
        node_pubkey,
        config_data.mint,
        signer.pubkey(),
        worker,
    );

    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, node_pubkey)
}
