use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

use super::utils::parse_pubkey;

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
        _ => Ok(CliCommand::TaskData {
            address: parse_pubkey(&"address".into(), matches)?,
        }),
    }
}
