use {
    crate::{
        cli::{CliCommand, CliError},
        config::CliConfig,
    },
    clap::ArgMatches,
};

pub fn process(matches: &ArgMatches) -> Result<(), CliError> {
    // Parse command
    let command = CliCommand::try_from(matches)?;
    let _config = CliConfig::load();

    // Process the command
    match command {
        CliCommand::Benchmark {
            count,
            parallelism,
            recurrence,
        } => super::benchmark::run(count, parallelism, recurrence),
    }
}
