use anchor_lang::{
    solana_program::{
        instruction::Instruction, pubkey::Pubkey,
    },
    InstructionData, ToAccountMetas
};
use clockwork_network_program::state::{Config, ConfigSettings};

use crate::{client::Client, errors::CliError};

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
    epoch_thread: Option<Pubkey>,
    hasher_thread: Option<Pubkey>,
) -> Result<(), CliError> {
    // Get the current config.
    let config = client
        .get::<Config>(&Config::pubkey())
        .map_err(|_err| CliError::AccountNotFound(Config::pubkey().to_string()))?;

    // Build new config. settings
    let settings = ConfigSettings {
        admin: admin.unwrap_or(config.admin),
        epoch_thread: epoch_thread.unwrap_or(config.epoch_thread),
        hasher_thread: hasher_thread.unwrap_or(config.hasher_thread),
        mint: config.mint,
    };

    // Submit tx
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::ConfigUpdate {
            admin: client.payer_pubkey(),
            config: Config::pubkey(),
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::ConfigUpdate { settings }.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client)?;
    Ok(())
}
