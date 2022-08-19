use {
    crate::errors::CliError,
    clockwork_client::{crank::state::Queue, Client},
    solana_sdk::pubkey::Pubkey,
};

pub fn get(client: &Client, address: &Pubkey) -> Result<(), CliError> {
    let queue = client
        .get::<Queue>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", queue);

    Ok(())
}
