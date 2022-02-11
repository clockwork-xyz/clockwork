pub mod app;
pub mod command;
pub mod config;
pub mod error;
pub mod process;

use solana_clap_utils::keypair::DefaultSigner;

use crate::app::cronos_app;
use crate::command::CliCommand;
use crate::config::CliConfig;
use crate::error::CliError;
use crate::process::process_command;

fn main() -> Result<(), CliError> {
    let matches = cronos_app().get_matches();
    let command = CliCommand::try_from(&matches)?;
    let config = CliConfig::try_from(&matches)?;
    let signer = DefaultSigner::new("keypair".to_string(), &config.keypair_path);
    process_command(&command, &config, &signer)
}
