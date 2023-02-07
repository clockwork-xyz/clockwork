use {
    crate::errors::CliError,
    clockwork_client::{
        network::state::{Config, ConfigSettings},
        Client,
    },
    solana_sdk::pubkey::Pubkey,
};

pub fn get(client: &Client) -> Result<(), CliError> {
    let config = client
        .get::<Config>(&Config::pubkey())
        .map_err(|_err| CliError::AccountNotFound(Config::pubkey().to_string()))?;
    println!("{:#?}", config);
    Ok(())
}

pub fn set(
    client: &Client,
    admin: Option<Pubkey>,
    epoch_automation: Option<Pubkey>,
    hasher_automation: Option<Pubkey>,
) -> Result<(), CliError> {
    // Get the current config.
    let config = client
        .get::<Config>(&Config::pubkey())
        .map_err(|_err| CliError::AccountNotFound(Config::pubkey().to_string()))?;

    // Build new config. settings
    let settings = ConfigSettings {
        admin: admin.unwrap_or(config.admin),
        epoch_automation: epoch_automation.unwrap_or(config.epoch_automation),
        hasher_automation: hasher_automation.unwrap_or(config.hasher_automation),
        mint: config.mint,
    };

    // Submit tx
    let ix = clockwork_client::network::instruction::config_update(client.payer_pubkey(), settings);
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client)?;
    Ok(())
}
