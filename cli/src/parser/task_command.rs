use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

use super::utils::{parse_i64_optional, parse_instruction, parse_pubkey, parse_string};

pub fn task_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("cancel", matches)) => {
            let address = parse_pubkey(&String::from("address"), matches)?;
            Ok(CliCommand::TaskCancel { address })
        }
        Some(("new", matches)) => {
            let filepath = parse_string(&String::from("filepath"), matches)?;
            let ix = parse_instruction(&filepath)?;
            let exec_at = parse_i64_optional(&String::from("exec_at"), matches)?;
            let stop_at = parse_i64_optional(&String::from("stop_at"), matches)?;
            let recurr = parse_i64_optional(&String::from("recurr"), matches)?;
            Ok(CliCommand::TaskNew {
                ix,
                exec_at,
                stop_at,
                recurr,
            })
        }
        _ => Ok(CliCommand::TaskGet {
            address: parse_pubkey(&String::from("address"), matches)?,
        }),
    }
}
