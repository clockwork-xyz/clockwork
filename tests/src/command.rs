use clap::ArgMatches;
use std::{convert::TryFrom, fmt::Display};

use crate::{error::TestError, parser::*};

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    Bench { count: u32, time: u32, percent: f32 },
}

impl Display for CliCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliCommand::Bench { .. } => write!(f, "bench"),
        }
    }
}

impl TryFrom<&ArgMatches> for CliCommand {
    type Error = TestError;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("bench", matches)) => bench_command(matches),
            _ => Err(TestError::CommandNotRecognized(
                matches.subcommand().unwrap().0.into(),
            )),
        }
    }
}
