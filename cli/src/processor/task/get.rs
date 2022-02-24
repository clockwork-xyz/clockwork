use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use cronos_sdk::account::Task;
use solana_client_helpers::Client;

use crate::{
    error::CliError,
    utils::{solana_explorer_url, SolanaExplorerAccountType},
};

pub fn get(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let task_data = Task::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "Explorer: {}",
        solana_explorer_url(SolanaExplorerAccountType::Account, address.to_string())
    );
    println!("{:#?}", task_data);
    Ok(())
}
