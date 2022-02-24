use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{
    error::CliError,
    utils::{solana_explorer_url, SolanaExplorerAccountType},
};

pub fn get(client: &Arc<Client>) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let daemon_addr = cronos_sdk::account::Daemon::pda(owner).0;
    let data = client
        .get_account_data(&daemon_addr)
        .map_err(|_err| CliError::AccountNotFound(daemon_addr.to_string()))?;
    let daemon_data = cronos_sdk::account::Daemon::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(daemon_addr.to_string()))?;
    println!(
        "Explorer: {}",
        solana_explorer_url(SolanaExplorerAccountType::Account, daemon_addr.to_string())
    );
    println!("{:#?}", daemon_data);
    Ok(())
}
