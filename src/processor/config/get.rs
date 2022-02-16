use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{
    error::CliError,
    utils::{explorer_url, ExplorerEntity},
};

pub fn get(client: &Arc<Client>) -> Result<(), CliError> {
    let config_addr = cronos_sdk::account::Config::find_pda().0;
    let data = client
        .get_account_data(&config_addr)
        .map_err(|_err| CliError::AccountNotFound(config_addr.to_string()))?;
    let config_data = cronos_sdk::account::Config::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_addr.to_string()))?;
    println!(
        "\n\n{}\n\n{}\n\n",
        explorer_url(ExplorerEntity::Account, config_addr.to_string()),
        config_data
    );
    Ok(())
}
