use clap::ArgMatches;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use std::{convert::TryFrom, fmt::Display};

use crate::{error::CliError, parser::*};

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    AdminHealthReset,
    AdminTaskCancel { address: Pubkey },
    AdminInitialize,
    ClockGet,
    ConfigGet,
    DaemonGet,
    DaemonNew,
    HealthGet,
    TaskCancel { address: Pubkey },
    TaskGet { address: Pubkey },
    TaskNew { ix: Instruction, schedule: String },
}

impl Display for CliCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliCommand::AdminHealthReset => write!(f, "admin health reset"),
            CliCommand::AdminInitialize => write!(f, "admin initialize"),
            CliCommand::AdminTaskCancel { address } => write!(f, "admin task cancel {}", address),
            CliCommand::ClockGet => write!(f, "clock"),
            CliCommand::ConfigGet => write!(f, "config"),
            CliCommand::DaemonGet => write!(f, "daemon"),
            CliCommand::DaemonNew => write!(f, "daemon new"),
            CliCommand::HealthGet => write!(f, "health"),
            CliCommand::TaskCancel { address } => write!(f, "task cancel {}", address),
            CliCommand::TaskGet { address } => write!(f, "task {}", address),
            CliCommand::TaskNew { .. } => write!(f, "task new"),
        }
    }
}

impl TryFrom<&ArgMatches> for CliCommand {
    type Error = CliError;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("admin", matches)) => admin_command(matches),
            Some(("clock", _matches)) => Ok(CliCommand::ClockGet {}),
            Some(("config", matches)) => config_command(matches),
            Some(("daemon", matches)) => daemon_command(matches),
            Some(("health", matches)) => health_command(matches),
            Some(("task", matches)) => task_command(matches),
            _ => Err(CliError::CommandNotRecognized(
                matches.subcommand().unwrap().0.into(),
            )),
        }
    }
}
