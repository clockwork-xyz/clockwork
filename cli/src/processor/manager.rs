use cronos_sdk::Client;
use solana_sdk::pubkey::Pubkey;

use crate::cli::CliError;

pub fn create(client: &Client) -> Result<(), CliError> {
    let authority = client.payer_pubkey();
    let manager_pubkey = cronos_sdk::scheduler::state::Manager::pda(authority).0;
    let ix = cronos_sdk::scheduler::instruction::manager_new(authority, authority, manager_pubkey);
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, &manager_pubkey)
}

pub fn get(client: &Client, address: &Pubkey) -> Result<(), CliError> {
    let manager = client
        .get::<cronos_sdk::scheduler::state::Manager>(address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", manager);
    Ok(())
}
