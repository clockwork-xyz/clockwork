use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use cronos_sdk::account::Task;
use solana_client_helpers::Client;

use crate::{error::CliError, utils::*};

pub fn data(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let task_data = Task::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "\n\n{}\n\n{}\n\n",
        explorer_url(ExplorerEntity::Account, address.to_string()),
        task_data
    );
    Ok(())
}
