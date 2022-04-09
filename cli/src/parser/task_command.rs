use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

use super::utils::{parse_instruction, parse_pubkey, parse_string};

pub fn task_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("cancel", matches)) => {
            let address = parse_pubkey(&String::from("address"), matches)?;
            Ok(CliCommand::TaskCancel { address })
        }
        Some(("new", matches)) => {
            let filepath = parse_string(&String::from("filepath"), matches)?;
            let ix = parse_instruction(&filepath)?;
            let schedule = parse_string(&String::from("schedule"), matches)?;
            Ok(CliCommand::TaskNew { ix, schedule })
        }
        _ => Ok(CliCommand::TaskGet {
            address: parse_pubkey(&String::from("address"), matches)?,
        }),
    }
}
