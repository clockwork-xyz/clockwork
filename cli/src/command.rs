use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;
use solana_sdk::instruction::Instruction;
use std::{convert::TryFrom, fmt::Display};

use crate::{error::CliError, parser::*};

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    AdminCancelTask { address: Pubkey },
    AdminHealthReset,
    AdminHealthStart,
    AdminInitialize,
    Clock,
    ConfigGet,
    ConfigSetMinRecurr { new_value: i64 },
    ConfigSetProgramFee { new_value: u64 },
    ConfigSetWorkerExecFee { new_value: u64 },
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
            CliCommand::AdminCancelTask { address } => write!(f, "admin cancel {}", address),
            CliCommand::AdminHealthReset => write!(f, "admin health reset"),
            CliCommand::AdminHealthStart => write!(f, "admin health start"),
            CliCommand::AdminInitialize => write!(f, "admin initialize"),
            CliCommand::Clock => write!(f, "clock"),
            CliCommand::ConfigGet => write!(f, "config"),
            CliCommand::ConfigSetMinRecurr { new_value } => {
                write!(f, "config set min_recurr {}", new_value)
            }
            CliCommand::ConfigSetProgramFee { new_value } => {
                write!(f, "config set program_fee {}", new_value)
            }
            CliCommand::ConfigSetWorkerExecFee { new_value } => {
                write!(f, "config set worker_exec_fee {}", new_value)
            }
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
