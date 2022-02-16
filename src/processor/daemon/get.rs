use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{
    error::CliError,
    utils::{explorer_url, ExplorerEntity},
};

pub fn get(client: &Arc<Client>) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let daemon_addr = cronos_sdk::account::Daemon::find_pda(owner).0;
    let data = client
        .get_account_data(&daemon_addr)
        .map_err(|_err| CliError::AccountNotFound(daemon_addr.to_string()))?;
    let daemon_data = cronos_sdk::account::Daemon::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(daemon_addr.to_string()))?;
    println!(
        "\n\n{}\n\n{}\n\n",
        explorer_url(ExplorerEntity::Account, daemon_addr.to_string()),
        daemon_data
    );
    Ok(())
}
