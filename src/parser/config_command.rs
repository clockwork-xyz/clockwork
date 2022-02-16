use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

use super::utils::parse_i64;

pub fn config_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("set", matches)) => config_set_command(matches),
        _ => Ok(CliCommand::ConfigGet),
    }
}

fn config_set_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("min_recurr", matches)) => {
            // let new_value = matches
            //     .value_of("new_value")
            //     .ok_or(CliError::BadParameter("new_value".into()))?
            //     .parse::<i64>()
            //     .map_err(|_err| CliError::BadParameter("new_value".into()))?;
            Ok(CliCommand::ConfigSetMinRecurr {
                new_value: parse_i64(&"new_value".into(), matches)?,
            })
        }
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}
