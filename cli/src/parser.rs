use crate::{cli::CliCommand, errors::CliError};
use clap::ArgMatches;
use clockwork_client::http::state::HttpMethod;
use serde::{Deserialize as JsonDeserialize, Serialize as JsonSerialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};
use std::{convert::TryFrom, fs, path::PathBuf, str::FromStr};

impl TryFrom<&ArgMatches> for CliCommand {
    type Error = CliError;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("api", matches)) => parse_api_command(matches),
            Some(("config", matches)) => parse_config_command(matches),
            Some(("http", matches)) => parse_http_command(matches),
            Some(("initialize", matches)) => parse_initialize_command(matches),
            Some(("localnet", matches)) => parse_bpf_command(matches),
            Some(("node", matches)) => parse_node_command(matches),
            Some(("pool", _)) => Ok(CliCommand::PoolGet {}),
            Some(("queue", matches)) => parse_queue_command(matches),
            Some(("registry", _matches)) => Ok(CliCommand::RegistryGet {}),
            Some(("snapshot", matches)) => parse_snapshot_command(matches),
            _ => Err(CliError::CommandNotRecognized(
                matches.subcommand().unwrap().0.into(),
            )),
        }
    }
}

// Command parsers
fn parse_bpf_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    let mut program_infos = Vec::<ProgramInfo>::new();

    if let Some(values) = matches.values_of("bpf_program") {
        let values: Vec<&str> = values.collect::<Vec<_>>();
        for address_program in values.chunks(2) {
            match address_program {
                [address, program] => {
                    let address = address
                        .parse::<Pubkey>()
                        .or_else(|_| read_keypair_file(address).map(|keypair| keypair.pubkey()));

                    if address.is_err() {
                        return Err(CliError::InvalidAddress);
                    }

                    let program_path = PathBuf::from(program);

                    if !program_path.exists() {
                        return Err(CliError::InvalidProgramFile);
                    }

                    program_infos.push(ProgramInfo {
                        program_id: address.unwrap(),
                        program_path,
                    });
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(CliCommand::Localnet { program_infos })
}

fn parse_api_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", matches)) => Ok(CliCommand::ApiNew {
            ack_authority: parse_pubkey("ack_authority", matches)?,
            base_url: parse_string("base_url", matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_config_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("get", _)) => Ok(CliCommand::ConfigGet {}),
        Some(("set", matches)) => Ok(CliCommand::ConfigSet {
            admin: parse_pubkey("admin", matches).map_or(None, |v| Some(v)),
            crank_fee: parse_u64("crank_fee", matches).map_or(None, |v| Some(v)),
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_http_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    Ok(CliCommand::HttpRequestNew {
        api: parse_pubkey("api", matches)?,
        id: parse_string("id", matches)?,
        method: parse_http_method("method", matches)?,
        route: parse_string("route", matches)?,
    })
}

fn parse_initialize_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    Ok(CliCommand::Initialize {
        mint: parse_pubkey("mint", matches)?,
    })
}

fn parse_node_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("register", matches)) => Ok(CliCommand::NodeRegister {
            worker: parse_keypair_file("worker", matches)?,
        }),
        Some(("stake", matches)) => Ok(CliCommand::NodeStake {
            address: parse_pubkey("address", matches)?,
            amount: parse_u64("amount", matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_queue_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("get", matches)) => Ok(CliCommand::QueueGet {
            address: parse_pubkey("address", matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_snapshot_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("get", matches)) => Ok(CliCommand::SnapshotGet {
            entry_id: match matches.subcommand() {
                Some(("entry", matches)) => Some(parse_u64("id", matches)?),
                _ => None,
            },
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

fn parse_http_method(arg: &str, matches: &ArgMatches) -> Result<HttpMethod, CliError> {
    Ok(HttpMethod::from_str(parse_string(arg, matches)?.as_str())
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

pub fn _parse_i64(arg: &str, matches: &ArgMatches) -> Result<i64, CliError> {
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

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramInfo {
    pub program_id: Pubkey,
    pub program_path: PathBuf,
}
