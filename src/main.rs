pub mod app;
pub mod command;
pub mod config;
pub mod error;

use solana_clap_utils::keypair::DefaultSigner;

use crate::app::cronos_app;
use crate::command::CliCommand;
use crate::config::CliConfig;
use crate::error::CliError;

fn main() -> Result<(), CliError> {
    let matches = cronos_app().get_matches();
    let command = CliCommand::try_from(&matches)?;
    let config = CliConfig::try_from(&matches)?;
    let default_signer = DefaultSigner::new("keypair".to_string(), &config.keypair_path);
    println!("Signer: {:?}", default_signer);
    println!("Config: {:?}", config.json_rpc_url);
    println!("Command: {:?}", command);
    Ok(())
}
