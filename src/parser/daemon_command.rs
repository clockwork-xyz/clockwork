use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

pub fn daemon_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", _matches)) => Ok(CliCommand::DaemonNew {}),
        _ => Ok(CliCommand::DaemonGet {}),
    }
}
