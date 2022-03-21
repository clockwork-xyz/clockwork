use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

pub fn daemon_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("open", _matches)) => Ok(CliCommand::DaemonOpen {}),
        _ => Ok(CliCommand::DaemonGet {}),
    }
}
