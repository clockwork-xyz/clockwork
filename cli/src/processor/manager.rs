use cronos_client::Client;
use solana_sdk::pubkey::Pubkey;

use crate::errors::CliError;

pub fn create(client: &Client) -> Result<(), CliError> {
    let authority = client.payer_pubkey();
    let manager_pubkey = cronos_client::scheduler::state::Manager::pubkey(authority);
    let ix =
        cronos_client::scheduler::instruction::manager_new(authority, authority, manager_pubkey);
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, &manager_pubkey)
}

pub fn get(client: &Client, address: &Pubkey) -> Result<(), CliError> {
    let manager = client
        .get::<cronos_client::scheduler::state::Manager>(address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", manager);
    Ok(())
}
