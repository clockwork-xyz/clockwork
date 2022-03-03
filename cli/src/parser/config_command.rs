use clap::ArgMatches;

use crate::{command::CliCommand, error::CliError};

use super::utils::{parse_i64, parse_u64};

pub fn config_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("set", matches)) => config_set_command(matches),
        _ => Ok(CliCommand::ConfigGet),
    }
}

fn config_set_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("min_recurr", matches)) => Ok(CliCommand::ConfigSetMinRecurr {
            new_value: parse_i64(&"new_value".into(), matches)?,
        }),
        Some(("program_fee", matches)) => Ok(CliCommand::ConfigSetProgramFee {
            new_value: parse_u64(&"new_value".into(), matches)?,
        }),
        Some(("worker_exec_fee", matches)) => Ok(CliCommand::ConfigSetWorkerExecFee {
            new_value: parse_u64(&"new_value".into(), matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}
