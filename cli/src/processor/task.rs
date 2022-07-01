use cronos_client::Client;
use solana_sdk::pubkey::Pubkey;

use crate::errors::CliError;

pub fn get(client: &Client, address: &Pubkey) -> Result<(), CliError> {
    let task = client
        .get::<cronos_client::scheduler::state::Task>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", task);
    Ok(())
}
