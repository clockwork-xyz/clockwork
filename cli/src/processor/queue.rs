use {
    crate::errors::CliError,
    clockwork_client::{queue::objects::Queue, Client},
    solana_sdk::pubkey::Pubkey,
};

pub fn get(client: &Client, address: &Pubkey) -> Result<(), CliError> {
    let queue = client
        .get::<Queue>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", queue);

    Ok(())
}

pub fn update(client: &Client, address: &Pubkey, rate_limit: Option<u64>) -> Result<(), CliError> {
    let ix = clockwork_client::queue::instruction::queue_update(
        client.payer_pubkey(),
        *address,
        None,
        rate_limit,
        None,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}
