use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

pub fn task_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", matches)) => new_task_command(matches),
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

fn new_task_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
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
