pub mod app;
pub mod command;
pub mod config;
pub mod error;
pub mod parse;
pub mod process;
pub mod signer;

use crate::app::cronos_app;
use crate::command::CliCommand;
use crate::config::CliConfig;
use crate::error::CliError;
use crate::process::process_command;

fn main() -> Result<(), CliError> {
    let matches = cronos_app().get_matches();
    let command = CliCommand::try_from(&matches)?;
    let config = CliConfig::try_from(&matches)?;
    process_command(command, config)
}
