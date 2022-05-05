use crate::cli::CliError;
use solana_client_helpers::Client;
use std::sync::Arc;

pub fn get(_client: &Arc<Client>) -> Result<(), CliError> {
    panic!("Not implemented")
    // let time =
    //     cronos_sdk::clock::get_time(client).map_err(|err| CliError::BadClient(err.to_string()))?;
    // println!("Clock: {}", time);
    // Ok(())
}
