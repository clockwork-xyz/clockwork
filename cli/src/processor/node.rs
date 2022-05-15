use {
    crate::{
        cli::CliError,
        utils::{sign_and_submit, solana_explorer_url, SolanaExplorerAccountType},
    },
    cronos_sdk::network::state::{Authority, Config, Node, Registry, Snapshot, SnapshotEntry},
    solana_client_helpers::Client,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    },
    std::sync::Arc,
};

pub fn get(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(&address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let data = Node::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "Explorer: {}",
        solana_explorer_url(SolanaExplorerAccountType::Account, address.to_string())
    );
    println!("{:#?}", data);
    Ok(())
}

pub fn register(client: &Arc<Client>, delegate: Keypair) -> Result<(), CliError> {
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

    let queue_pubkey = cronos_sdk::scheduler::state::Queue::pda(authority_pubkey).0;
    let cycler_task_pubkey = cronos_sdk::scheduler::state::Task::pda(queue_pubkey, 0).0;
    let cycler_action_pubkey = cronos_sdk::scheduler::state::Action::pda(
        cycler_task_pubkey,
        registry_data.node_count.into(),
    )
    .0;

    let snapshot_task_pubkey = cronos_sdk::scheduler::state::Task::pda(queue_pubkey, 1).0;
    let snapshot_action_pubkey = cronos_sdk::scheduler::state::Action::pda(
        snapshot_task_pubkey,
        (registry_data.node_count + 1).into(),
    )
    .0;

    let ix = cronos_sdk::network::instruction::node_register(
        authority_pubkey,
        config_pubkey,
        delegate.pubkey(),
        entry_pubkey,
        config_data.mint,
        node_pubkey,
        owner.pubkey(),
        registry_pubkey,
        snapshot_pubkey,
        // Additional accounts
        cycler_action_pubkey,
        cycler_task_pubkey,
        queue_pubkey,
        snapshot_action_pubkey,
        snapshot_task_pubkey,
    );
    sign_and_submit(client, &[ix], &[owner, &delegate]);
    get(client, &node_pubkey)
}

pub fn stake(client: &Arc<Client>, amount: u64, delegate: Pubkey) -> Result<(), CliError> {
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
    let ix = cronos_sdk::network::instruction::node_stake(
        amount,
        config_pubkey,
        delegate,
        node_pubkey,
        config_data.mint,
        signer.pubkey(),
    );

    println!("Stake ix: {:#?}", ix);

    sign_and_submit(client, &[ix], &[client.payer()]);
    get(client, &node_pubkey)
}
