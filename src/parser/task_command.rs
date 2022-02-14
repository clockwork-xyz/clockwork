use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

pub fn task_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", _matches)) => {
            // TODO parse arguments
            let exec_at = None;
            let stop_at = None;
            let recurr = None;
            Ok(CliCommand::TaskNew {
                exec_at,
                stop_at,
                recurr,
            })
        }
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
