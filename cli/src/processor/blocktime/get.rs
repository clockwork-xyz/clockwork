use std::sync::Arc;

use solana_client_helpers::Client;

use crate::error::CliError;

pub fn get(client: &Arc<Client>) -> Result<(), CliError> {
    let blocktime =
        cronos_sdk::blocktime(client).map_err(|err| CliError::BadClient(err.to_string()))?;
    println!("Blocktime: {}", blocktime);
    Ok(())
}
