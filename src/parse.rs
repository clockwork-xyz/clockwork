use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;

use super::error::CliError;

use super::command::CliCommand;

pub fn parse_daemon_app_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", _matches)) => Ok(CliCommand::DaemonNew {}),
        _ => Ok(CliCommand::DaemonData {}),
        // Some(("data", _matches)) => Ok(CliCommand::DaemonData {}),
        // _ => Err(CliError::CommandNotRecognized(
        //     matches.subcommand().unwrap().0.into(),
        // )),
    }
}

pub fn parse_health_app_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        _ => Ok(CliCommand::HealthCheck),
    }
}

pub fn parse_task_app_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", matches)) => parse_task_new_app_command(matches),
        _ => {
            let address = matches
                .value_of("address")
                .ok_or(CliError::BadParameter("address".into()))?;
            Ok(CliCommand::TaskData {
                address: Pubkey::from_str(address)
                    .map_err(|_err| CliError::BadParameter("address".into()))?,
            })
        }
    }
}

pub fn parse_task_new_app_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("memo", matches)) => {
            let memo = matches
                .value_of("memo")
                .ok_or(CliError::BadParameter("memo".into()))?
                .to_string();

            // TODO parse arguments
            let exec_at = None;
            let stop_at = None;
            let recurr = None;
            Ok(CliCommand::TaskNewMemo {
                memo,
                exec_at,
                stop_at,
                recurr,
            })
        }
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}
