use {
    crate::{
        cli::CliError,
        utils::{sign_and_submit, solana_explorer_url, SolanaExplorerAccountType},
    },
    cronos_sdk::network::state::{Authority, Config, Node, Registry},
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

pub fn register(client: &Arc<Client>, _delegate: Pubkey) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pda().0;
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config_data = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Build ix
    let delegate = &Keypair::new(); // TODO Load from keypair from filepath and make it a signer
    let owner = client.payer().clone();
    let authority_pubkey = Authority::pda().0;
    let node_pubkey = Node::pda(delegate.pubkey()).0;
    let registry_pubkey = Registry::pda().0;
    let queue_pubkey = cronos_sdk::scheduler::state::Queue::pda(authority_pubkey).0;
    let task_pubkey = cronos_sdk::scheduler::state::Task::pda(queue_pubkey, 0).0;
    let task_data = client
        .get_account_data(&task_pubkey)
        .map_err(|_err| CliError::AccountNotFound(task_pubkey.to_string()))?;
    let task_data = cronos_sdk::scheduler::state::Task::try_from(task_data)
        .map_err(|_err| CliError::AccountDataNotParsable(task_pubkey.to_string()))?;
    let action_pubkey =
        cronos_sdk::scheduler::state::Action::pda(task_pubkey, task_data.action_count).0;

    let ix = cronos_sdk::network::instruction::register(
        authority_pubkey,
        config_pubkey,
        delegate.pubkey(),
        config_data.mint,
        node_pubkey,
        owner.pubkey(),
        registry_pubkey,
        action_pubkey,
        queue_pubkey,
        task_pubkey,
    );
    // sign_and_submit(client, &[ix]);
    sign_and_submit(client, &[ix], &[owner, delegate]);
    get(client, &node_pubkey)
}

pub fn stake(client: &Arc<Client>, amount: u64) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pda().0;
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config_data = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Build ix
    let identity = client.payer_pubkey();
    let node_pubkey = Node::pda(identity).0;
    let ix = cronos_sdk::network::instruction::stake(
        amount,
        config_pubkey,
        identity,
        config_data.mint,
        node_pubkey,
    );
    sign_and_submit(client, &[ix], &[client.payer()]);
    get(client, &node_pubkey)
}
