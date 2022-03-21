use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;
use solana_sdk::instruction::Instruction;
use std::{convert::TryFrom, fmt::Display};

use crate::{error::CliError, parser::*};

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    AdminHealthReset,
    AdminOpen,
    AdminTaskClose { address: Pubkey },
    Clock,
    ConfigGet,
    DaemonGet,
    DaemonOpen,
    HealthGet,
    TaskClose { address: Pubkey },
    TaskGet { address: Pubkey },
    TaskOpen { ix: Instruction, schedule: String },
}

impl Display for CliCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliCommand::AdminHealthReset => write!(f, "admin health reset"),
            CliCommand::AdminOpen => write!(f, "admin open"),
            CliCommand::AdminTaskClose { address } => write!(f, "admin task close {}", address),
            CliCommand::Clock => write!(f, "clock"),
            CliCommand::ConfigGet => write!(f, "config"),
            CliCommand::DaemonGet => write!(f, "daemon"),
            CliCommand::DaemonOpen => write!(f, "daemon open"),
            CliCommand::HealthGet => write!(f, "health"),
            CliCommand::TaskClose { address } => write!(f, "task close {}", address),
            CliCommand::TaskGet { address } => write!(f, "task {}", address),
            CliCommand::TaskOpen { .. } => write!(f, "task open"),
        }
    }
}

impl TryFrom<&ArgMatches> for CliCommand {
    type Error = CliError;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("admin", matches)) => admin_command(matches),
            Some(("clock", _matches)) => Ok(CliCommand::Clock {}),
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
