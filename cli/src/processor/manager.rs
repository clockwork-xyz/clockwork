use std::sync::Arc;

use solana_client_helpers::Client;
use solana_sdk::pubkey::Pubkey;

use crate::{
    cli::CliError,
    utils::{sign_and_submit, solana_explorer_url, SolanaExplorerAccountType},
};

pub fn create(client: &Arc<Client>) -> Result<(), CliError> {
    let authority = client.payer_pubkey();
    let manager_pubkey = cronos_sdk::scheduler::state::Manager::pda(authority).0;
    let ix = cronos_sdk::scheduler::instruction::manager_new(authority, authority, manager_pubkey);
    sign_and_submit(client, &[ix], &[client.payer()]);
    get(client, &manager_pubkey)
}

pub fn get(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(&address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let manager_data = cronos_sdk::scheduler::state::Manager::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "Explorer: {}",
        solana_explorer_url(SolanaExplorerAccountType::Account, address.to_string())
    );
    println!("{:#?}", manager_data);
    Ok(())
}
