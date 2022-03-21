use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

pub fn config_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        _ => Ok(CliCommand::ConfigGet),
    }
}
