use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use cronos_sdk::account::Task;
use solana_client_helpers::Client;

use crate::error::CliError;

pub fn process(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let task_data = Task::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "{{
    daemon: {},
    id: {},
    status: {},
    exec_at: {},
    stop_at: {},
    recurr: {},
    ix: {:?}
}}",
        task_data.daemon,
        task_data.id,
        task_data.status,
        task_data.exec_at,
        task_data.stop_at,
        task_data.recurr,
        task_data.ix,
    );
    Ok(())
}
