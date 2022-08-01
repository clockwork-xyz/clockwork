use clockwork_client::network::state::Registry;

use {crate::errors::CliError, clockwork_client::Client};

pub fn get(client: &Client) -> Result<(), CliError> {
    let registry_pubkey = clockwork_client::network::state::Registry::pubkey();
    let registry = client
        .get::<Registry>(&registry_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;
    println!("{:#?}", registry);
    Ok(())
}
