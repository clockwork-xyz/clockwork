use clap::ArgMatches;

use crate::{command::CliCommand, error::TestError};

pub fn admin_command(matches: &ArgMatches) -> Result<CliCommand, TestError> {
    match matches.subcommand() {
        Some(("initialize", _matches)) => Ok(CliCommand::AdminInitialize),
        _ => Err(TestError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}
