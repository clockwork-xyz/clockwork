use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{
    error::CliError,
    utils::{solana_explorer_url, SolanaExplorerAccountType},
};

pub fn get(client: &Arc<Client>) -> Result<(), CliError> {
    let config_addr = cronos_sdk::cronos::state::Config::pda().0;
    let data = client
        .get_account_data(&config_addr)
        .map_err(|_err| CliError::AccountNotFound(config_addr.to_string()))?;
    let config_data = cronos_sdk::cronos::state::Config::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_addr.to_string()))?;
    println!(
        "{}",
        solana_explorer_url(SolanaExplorerAccountType::Account, config_addr.to_string()),
    );
    println!("{:#?}", config_data);
    Ok(())
}
