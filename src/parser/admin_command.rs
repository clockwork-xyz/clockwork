use clap::ArgMatches;

use super::utils::parse_pubkey;
use crate::{command::CliCommand, error::CliError};

pub fn admin_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("cancel", matches)) => Ok(CliCommand::AdminCancelTask {
            address: parse_pubkey(&"address".into(), matches)?,
        }),
        Some(("health", _matches)) => Ok(CliCommand::AdminScheduleHealthCheck),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}
