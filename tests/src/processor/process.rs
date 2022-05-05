use crate::{command::CliCommand, config::CliConfig, error::TestError};
use clap::ArgMatches;

pub fn process(matches: &ArgMatches) -> Result<(), TestError> {
    // Parse command and config
    let command = CliCommand::try_from(matches)?;
    let _config = CliConfig::load();

    // Process the command
    match command {
        CliCommand::Bench {
            count,
            time,
            percent,
        } => super::bench::test(count, time, percent),
    }
}
