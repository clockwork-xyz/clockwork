use clockwork_client::network::objects::Registry;

use {crate::errors::CliError, clockwork_client::Client};

pub fn get(client: &Client) -> Result<(), CliError> {
    let registry_pubkey = clockwork_client::network::objects::Registry::pubkey();
    let registry = client
        .get::<Registry>(&registry_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;
    println!("{:#?}", registry);
    Ok(())
}

pub fn unlock(client: &Client) -> Result<(), CliError> {
    let ix = clockwork_client::network::instruction::registry_unlock(client.payer_pubkey());
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client)?;
    Ok(())
}
