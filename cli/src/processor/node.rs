use {
    crate::{
        cli::CliError,
        utils::{sign_and_submit, solana_explorer_url, SolanaExplorerAccountType},
    },
    cronos_sdk::network::state::{Config, Node, Registry},
    solana_client_helpers::Client,
    solana_sdk::pubkey::Pubkey,
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

pub fn register(client: &Arc<Client>) -> Result<(), CliError> {
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
    let registry_pubkey = Registry::pda().0;
    let ix = cronos_sdk::network::instruction::register(
        config_pubkey,
        identity,
        config_data.mint,
        node_pubkey,
        registry_pubkey,
    );
    sign_and_submit(client, &[ix]);
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
    sign_and_submit(client, &[ix]);
    get(client, &node_pubkey)
}
