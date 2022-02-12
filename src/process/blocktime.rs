use std::sync::Arc;

use solana_client_helpers::Client;

use crate::error::CliError;

pub fn process(client: &Arc<Client>) -> Result<(), CliError> {
    let blocktime = cronos_sdk::blocktime::blocktime(client)
        .map_err(|err| CliError::BadClient(err.to_string()))?;
    println!("{}", blocktime);
    Ok(())
}
