use crate::cli::{CliCommand, CliError};
use clap::ArgMatches;
use serde::{Deserialize as JsonDeserialize, Serialize as JsonSerialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
};
use std::str::FromStr;
use std::{convert::TryFrom, fs};

impl TryFrom<&ArgMatches> for CliCommand {
    type Error = CliError;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("task", matches)) => parse_task_command(matches),
            Some(("clock", _)) => Ok(CliCommand::Clock {}),
            Some(("config", matches)) => parse_config_command(matches),
            Some(("health", _)) => Ok(CliCommand::Health {}),
            Some(("initialize", matches)) => parse_initialize_command(matches),
            Some(("manager", matches)) => parse_manager_command(matches),
            Some(("node", matches)) => parse_node_command(matches),
            Some(("pool", _)) => Ok(CliCommand::PoolGet {}),
            Some(("queue", matches)) => parse_queue_command(matches),
            Some(("registry", _matches)) => Ok(CliCommand::RegistryGet {}),
            _ => Err(CliError::CommandNotRecognized(
                matches.subcommand().unwrap().0.into(),
            )),
        }
    }
}

// Command parsers

fn parse_config_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("get", _)) => Ok(CliCommand::ConfigGet {}),
        Some(("set", matches)) => Ok(CliCommand::ConfigSet {
            admin: parse_pubkey("admin", matches).map_or(None, |v| Some(v)),
            delegate_fee: parse_u64("delegate_fee", matches).map_or(None, |v| Some(v)),
            delegate_holdout_period: parse_i64("delegate_holdout_period", matches)
                .map_or(None, |v| Some(v)),
            delegate_spam_penalty: parse_u64("delegate_spam_penalty", matches)
                .map_or(None, |v| Some(v)),
            program_fee: parse_u64("program_fee", matches).map_or(None, |v| Some(v)),
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_task_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("get", matches)) => Ok(CliCommand::TaskGet {
            address: parse_pubkey("address", matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_initialize_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    Ok(CliCommand::Initialize {
        mint: parse_pubkey("mint", matches)?,
    })
}

fn parse_node_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("register", matches)) => Ok(CliCommand::NodeRegister {
            delegate: parse_keypair_file("delegate", matches)?,
        }),
        Some(("stake", matches)) => Ok(CliCommand::NodeStake {
            amount: parse_u64("amount", matches)?,
            delegate: parse_pubkey("delegate", matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_manager_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("create", _)) => Ok(CliCommand::ManagerCreate {}),
        Some(("get", matches)) => Ok(CliCommand::ManagerGet {
            address: parse_pubkey("address", matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_queue_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("create", matches)) => Ok(CliCommand::QueueCreate {
            schedule: parse_string("schedule", matches)?,
        }),
        Some(("get", matches)) => Ok(CliCommand::QueueGet {
            address: parse_pubkey("address", matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

// Arg parsers

fn parse_keypair_file(arg: &str, matches: &ArgMatches) -> Result<Keypair, CliError> {
    Ok(read_keypair_file(parse_string(arg, matches)?)
        .map_err(|_err| CliError::BadParameter(arg.into()))?)
}

fn parse_pubkey(arg: &str, matches: &ArgMatches) -> Result<Pubkey, CliError> {
    Ok(Pubkey::from_str(parse_string(arg, matches)?.as_str())
        .map_err(|_err| CliError::BadParameter(arg.into()))?)
}

fn parse_string(arg: &str, matches: &ArgMatches) -> Result<String, CliError> {
    Ok(matches
        .value_of(arg)
        .ok_or(CliError::BadParameter(arg.into()))?
        .to_string())
}

pub fn parse_i64(arg: &str, matches: &ArgMatches) -> Result<i64, CliError> {
    Ok(parse_string(arg, matches)?
        .parse::<i64>()
        .map_err(|_err| CliError::BadParameter(arg.into()))
        .unwrap())
}

pub fn parse_u64(arg: &str, matches: &ArgMatches) -> Result<u64, CliError> {
    Ok(parse_string(arg, matches)?
        .parse::<u64>()
        .map_err(|_err| CliError::BadParameter(arg.into()))
        .unwrap())
}

// Json parsers

#[derive(Debug, JsonDeserialize, JsonSerialize)]
pub struct JsonInstructionData {
    pub program_id: String,
    pub accounts: Vec<JsonAccountMetaData>,
    pub data: Vec<u8>,
}

impl TryFrom<&JsonInstructionData> for Instruction {
    type Error = CliError;

    fn try_from(value: &JsonInstructionData) -> Result<Self, Self::Error> {
        Ok(Instruction {
            program_id: Pubkey::from_str(value.program_id.as_str())
                .map_err(|_err| CliError::BadParameter("asdf".into()))?,
            accounts: value
                .accounts
                .iter()
                .map(|ix| AccountMeta::try_from(ix).unwrap())
                .collect::<Vec<AccountMeta>>(),
            data: value.data.clone(),
        })
    }
}

pub fn _parse_instruction(filepath: &String) -> Result<Instruction, CliError> {
    let text =
        fs::read_to_string(filepath).map_err(|_err| CliError::BadParameter("filepath".into()))?;
    let ix: JsonInstructionData =
        serde_json::from_str(text.as_str()).expect("JSON was not well-formatted");
    Instruction::try_from(&ix)
}

#[derive(Debug, JsonDeserialize, JsonSerialize, PartialEq)]
pub struct JsonAccountMetaData {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl TryFrom<&JsonAccountMetaData> for AccountMeta {
    type Error = CliError;

    fn try_from(value: &JsonAccountMetaData) -> Result<Self, Self::Error> {
        Ok(AccountMeta {
            pubkey: Pubkey::from_str(value.pubkey.as_str())
                .map_err(|_err| CliError::BadParameter("asdf".into()))?,
            is_signer: value.is_signer,
            is_writable: value.is_writable,
        })
    }
}
