use clap::ArgMatches;

use crate::{command::CliCommand, error::TestError};

use super::utils::*;

pub fn bench_command(matches: &ArgMatches) -> Result<CliCommand, TestError> {
    match matches.subcommand() {
        _ => {
            let count = _parse_u32(&String::from("count"), matches)?;
            let time = _parse_u32(&String::from("time"), matches)?;
            let percent = _parse_f32(&String::from("percent"), matches)?;

            Ok(CliCommand::Bench {
                count,
                time,
                percent,
            })
        }
    }
}
