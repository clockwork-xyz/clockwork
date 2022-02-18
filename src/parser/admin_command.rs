use clap::ArgMatches;

use super::utils::parse_pubkey;
use crate::{command::CliCommand, error::CliError};

pub fn admin_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("cancel", matches)) => Ok(CliCommand::AdminCancelTask {
            address: parse_pubkey(&"address".into(), matches)?,
        }),
        Some(("health", matches)) => admin_health_command(matches),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn admin_health_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("reset", _matches)) => Ok(CliCommand::AdminHealthReset),
        Some(("start", _matches)) => Ok(CliCommand::AdminHealthStart),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}
