use std::sync::Arc;

use solana_client_helpers::Client;
use solana_sdk::pubkey::Pubkey;

use crate::{
    cli::CliError,
    utils::{sign_and_submit, solana_explorer_url, SolanaExplorerAccountType},
};

pub fn register(client: &Arc<Client>, _identity: Pubkey) -> Result<(), CliError> {
    // TODO identity should be a Signer
    let signer = client.payer_pubkey();
    let config_pda = cronos_sdk::network::state::Config::pda();
    let node_pda = cronos_sdk::network::state::Node::pda(signer);
    let registry_pda = cronos_sdk::network::state::Registry::pda();

    // Get config data
    let config_data = client
        .get_account_data(&config_pda.0)
        .map_err(|_err| CliError::AccountNotFound(config_pda.0.to_string()))?;
    let config_data = cronos_sdk::network::state::Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pda.0.to_string()))?;

    // Build ix
    let ix = cronos_sdk::network::instruction::register(
        config_pda.0,
        signer,
        node_pda,
        config_data.mint,
        registry_pda.0,
    );
    sign_and_submit(client, &[ix]);
    get(client, &node_pda.0)
}

pub fn get(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(&address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let data = cronos_sdk::network::state::Node::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "Explorer: {}",
        solana_explorer_url(SolanaExplorerAccountType::Account, address.to_string())
    );
    println!("{:#?}", data);
    Ok(())
}
