use {
    crate::errors::CliError,
    clockwork_client::{network::objects::Config, Client},
    solana_sdk::pubkey::Pubkey,
};

pub fn get(client: &Client) -> Result<(), CliError> {
    // Get network config
    let config_pubkey = Config::pubkey();
    let config = client
        .get::<Config>(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;

    // Print configs
    println!("{:#?}", config);

    Ok(())
}
