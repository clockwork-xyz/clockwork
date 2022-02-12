use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;

use super::error::CliError;

use super::command::CliCommand;

pub fn parse_daemon_app_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", _matches)) => Ok(CliCommand::DaemonNew {}),
        Some(("data", _matches)) => Ok(CliCommand::DaemonData {}),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

pub fn parse_health_app_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        _ => Ok(CliCommand::HealthCheck),
    }
}

pub fn parse_task_app_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", _matches)) => Ok(CliCommand::TaskNew {}),
        Some(("data", matches)) => {
            let address = matches
                .value_of("address")
                .ok_or(CliError::BadParameter("address".into()))?;
            Ok(CliCommand::TaskData {
                address: Pubkey::from_str(address)
                    .map_err(|_err| CliError::BadParameter("address".into()))?,
            })
        }
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}
