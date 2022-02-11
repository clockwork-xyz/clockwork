use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;
use std::{convert::TryFrom, str::FromStr};

use crate::error::CliError;

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    DaemonData,
    DaemonNew,
    Health,
    TaskData { address: Pubkey },
    TaskNew,
}

impl TryFrom<&ArgMatches> for CliCommand {
    type Error = CliError;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("daemon", matches)) => parse_daemon_command(matches),
            Some(("task", matches)) => parse_task_command(matches),
            _ => Err(CliError::CommandNotRecognized(
                matches.subcommand().unwrap().0.into(),
            )),
        }
    }
}

fn parse_daemon_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", _matches)) => Ok(CliCommand::DaemonNew {}),
        Some(("health", _matches)) => Ok(CliCommand::Health {}),
        Some(("data", _matches)) => Ok(CliCommand::DaemonData {}),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_task_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
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
