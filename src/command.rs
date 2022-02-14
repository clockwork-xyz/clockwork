use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;
use std::{convert::TryFrom, fmt::Display};

use crate::{error::CliError, parser::*};

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    Blocktime,
    DaemonData,
    DaemonNew,
    HealthCheck,
    TaskData {
        address: Pubkey,
    },
    TaskNewMemo {
        memo: String,
        exec_at: Option<i64>,
        stop_at: Option<i64>,
        recurr: Option<i64>,
    },
}

impl Display for CliCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliCommand::Blocktime => write!(f, "blocktime"),
            CliCommand::DaemonData => write!(f, "daemon"),
            CliCommand::DaemonNew => write!(f, "daemon new"),
            CliCommand::HealthCheck => write!(f, "health"),
            CliCommand::TaskData { address } => write!(f, "task {}", address),
            CliCommand::TaskNewMemo { .. } => write!(f, "task new memo"),
        }
    }
}

impl TryFrom<&ArgMatches> for CliCommand {
    type Error = CliError;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("blocktime", _matches)) => Ok(CliCommand::Blocktime {}),
            Some(("daemon", matches)) => daemon_command(matches),
            Some(("health", matches)) => health_command(matches),
            Some(("task", matches)) => task_command(matches),
            _ => Err(CliError::CommandNotRecognized(
                matches.subcommand().unwrap().0.into(),
            )),
        }
    }
}
