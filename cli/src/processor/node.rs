use {
    crate::cli::CliError,
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

pub fn get_by_delegate(client: &Client, delegate: Pubkey) -> Result<(), CliError> {
    let node_pubkey = Node::pda(delegate).0;
    get(client, node_pubkey)
}

pub fn register(client: &Client, delegate: Keypair) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pda().0;
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config_data = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Build ix
    let owner = client.payer().clone();
    let authority_pubkey = Authority::pda().0;
    let node_pubkey = Node::pda(delegate.pubkey()).0;
    let registry_pubkey = Registry::pda().0;
    let registry_data = client
        .get_account_data(&registry_pubkey)
        .map_err(|_err| CliError::AccountNotFound(registry_pubkey.to_string()))?;
    let registry_data = Registry::try_from(registry_data)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    let snapshot_pubkey = Snapshot::pda(registry_data.snapshot_count - 1).0;
    let entry_pubkey = SnapshotEntry::pda(snapshot_pubkey, registry_data.node_count).0;

    let manager_pubkey = cronos_client::scheduler::state::Manager::pda(authority_pubkey).0;
    // let rotator_queue_pubkey = cronos_client::scheduler::state::Queue::pda(manager_pubkey, 0).0;
    // let rotator_task_pubkey = cronos_client::scheduler::state::Task::pda(
    //     rotator_queue_pubkey,
    //     registry_data.node_count.into(),
    // )
    // .0;

    let snapshot_queue_pubkey = cronos_client::scheduler::state::Queue::pda(manager_pubkey, 0).0;
    let snapshot_task_pubkey = cronos_client::scheduler::state::Task::pda(
        snapshot_queue_pubkey,
        (registry_data.node_count + 1).into(),
    )
    .0;

    let ix = cronos_client::network::instruction::node_register(
        authority_pubkey,
        config_pubkey,
        // rotator_queue_pubkey,
        // rotator_task_pubkey,
        delegate.pubkey(),
        entry_pubkey,
        manager_pubkey,
        config_data.mint,
        node_pubkey,
        owner.pubkey(),
        registry_pubkey,
        snapshot_pubkey,
        snapshot_queue_pubkey,
        snapshot_task_pubkey,
    );
    client.send_and_confirm(&[ix], &[owner, &delegate]).unwrap();
    get(client, node_pubkey)
}

pub fn stake(client: &Client, amount: u64, delegate: Pubkey) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pda().0;
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config_data = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Build ix
    let signer = client.payer();
    let node_pubkey = Node::pda(delegate).0;
    let ix = cronos_client::network::instruction::node_stake(
        amount,
        config_pubkey,
        delegate,
        node_pubkey,
        config_data.mint,
        signer.pubkey(),
    );

    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, node_pubkey)
}
