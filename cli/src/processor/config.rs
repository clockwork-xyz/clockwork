use clockwork_client::network::objects::ConfigSettings;
use solana_sdk::pubkey::Pubkey;

use {
    crate::errors::CliError,
    clockwork_client::{network::objects::Config, Client},
};

pub fn get(client: &Client) -> Result<(), CliError> {
    let config = client
        .get::<Config>(&Config::pubkey())
        .map_err(|_err| CliError::AccountNotFound(Config::pubkey().to_string()))?;
    println!("{:#?}", config);
    Ok(())
}

pub fn _set(
    client: &Client,
    epoch_queue: Option<Pubkey>,
    hasher_queue: Option<Pubkey>,
) -> Result<(), CliError> {
    // Get the current config.
    let config = client
        .get::<Config>(&Config::pubkey())
        .map_err(|_err| CliError::AccountNotFound(Config::pubkey().to_string()))?;

    // Build new config. settings
    let settings = ConfigSettings {
        admin: config.admin,
        epoch_queue: epoch_queue.unwrap_or(config.epoch_queue),
        hasher_queue: hasher_queue.unwrap_or(config.hasher_queue),
        mint: config.mint,
    };

    // Submit tx
    let ix = clockwork_client::network::instruction::config_update(client.payer_pubkey(), settings);
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();

    println!("{:#?}", config);
    Ok(())
}
