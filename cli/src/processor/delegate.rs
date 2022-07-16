use cronos_client::Client;
use solana_sdk::pubkey::Pubkey;

use crate::errors::CliError;

pub fn create(client: &Client) -> Result<(), CliError> {
    let authority = client.payer_pubkey();
    let delegate_pubkey = cronos_client::scheduler::state::Delegate::pubkey(authority);
    let ix = cronos_client::scheduler::instruction::delegate_new(authority, authority);
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, &delegate_pubkey)
}

pub fn get(client: &Client, address: &Pubkey) -> Result<(), CliError> {
    let delegate = client
        .get::<cronos_client::scheduler::state::Delegate>(address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", delegate);
    Ok(())
}
