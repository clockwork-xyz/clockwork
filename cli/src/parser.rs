use std::{convert::TryFrom, fs, path::PathBuf, str::FromStr};

use clap::ArgMatches;
use clockwork_thread_program::state::{SerializableAccount, SerializableInstruction, Trigger};
use clockwork_webhook_program::state::HttpMethod;
use serde::{Deserialize as JsonDeserialize, Serialize as JsonSerialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};

use crate::{cli::CliCommand, errors::CliError};

impl TryFrom<&ArgMatches> for CliCommand {
    type Error = CliError;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("config", matches)) => parse_config_command(matches),
            Some(("crontab", matches)) => parse_crontab_command(matches),
            Some(("delegation", matches)) => parse_delegation_command(matches),
            Some(("explorer", matches)) => parse_explorer_command(matches),
            Some(("initialize", matches)) => parse_initialize_command(matches),
            Some(("localnet", matches)) => parse_bpf_command(matches),
            Some(("pool", matches)) => parse_pool_command(matches),
            Some(("secret", matches)) => parse_secret_command(matches),
            Some(("thread", matches)) => parse_thread_command(matches),
            Some(("registry", matches)) => parse_registry_command(matches),
            Some(("webhook", matches)) => parse_webhook_command(matches),
            Some(("worker", matches)) => parse_worker_command(matches),
            _ => Err(CliError::CommandNotRecognized(
                matches.subcommand().unwrap().0.into(),
            )),
        }
    }
}

// Command parsers
fn parse_bpf_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    let mut program_infos = Vec::<ProgramInfo>::new();
    let mut clone_addresses = Vec::<Pubkey>::new();

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

    if let Some(values) = matches.values_of("clone") {
        let values: Vec<&str> = values.collect::<Vec<_>>();
        for value in values {
            let address = value
                .parse::<Pubkey>()
                .map_err(|_| CliError::InvalidAddress)
                .unwrap();
            clone_addresses.push(address);
        }
    }

    Ok(CliCommand::Localnet {
        clone_addresses,
        network_url: parse_string("url", matches).ok(),
        program_infos,
        force_init: matches.is_present("force_init"),
        solana_archive: parse_string("solana_archive", matches).ok(),
        clockwork_archive: parse_string("clockwork_archive", matches).ok(),
        dev: matches.is_present("dev"),
    })
}

fn parse_config_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("get", _)) => Ok(CliCommand::ConfigGet {}),
        Some(("set", matches)) => Ok(CliCommand::ConfigSet {
            admin: parse_pubkey("admin", matches).ok(),
            epoch_thread: parse_pubkey("epoch_thread", matches).ok(),
            hasher_thread: parse_pubkey("hasher_thread", matches).ok(),
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_crontab_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    Ok(CliCommand::Crontab {
        schedule: parse_string("schedule", matches)?,
    })
}

fn parse_delegation_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("create", matches)) => Ok(CliCommand::DelegationCreate {
            worker_id: parse_u64("worker_id", matches)?,
        }),
        Some(("deposit", matches)) => Ok(CliCommand::DelegationDeposit {
            amount: parse_u64("amount", matches)?,
            delegation_id: parse_u64("delegation_id", matches)?,
            worker_id: parse_u64("worker_id", matches)?,
        }),
        Some(("get", matches)) => Ok(CliCommand::DelegationGet {
            delegation_id: parse_u64("delegation_id", matches)?,
            worker_id: parse_u64("worker_id", matches)?,
        }),
        Some(("withdraw", matches)) => Ok(CliCommand::DelegationWithdraw {
            amount: parse_u64("amount", matches)?,
            delegation_id: parse_u64("delegation_id", matches)?,
            worker_id: parse_u64("worker_id", matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_explorer_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("get", matches)) => Ok(CliCommand::ExplorerGetThread {
            id: parse_string("id", matches).ok(),
            address: parse_pubkey("address", matches).ok(),
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

fn parse_pool_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("get", matches)) => Ok(CliCommand::PoolGet {
            id: parse_u64("id", matches)?,
        }),
        Some(("update", matches)) => Ok(CliCommand::PoolUpdate {
            id: parse_u64("id", matches)?,
            size: parse_usize("size", matches)?,
        }),
        Some(("list", _)) => Ok(CliCommand::PoolList {}),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_secret_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("approve", matches)) => Ok(CliCommand::SecretApprove {
            name: parse_string("name", matches)?,
            delegate: parse_pubkey("delegate", matches)?,
        }),
        Some(("get", matches)) => Ok(CliCommand::SecretGet {
            name: parse_string("name", matches)?,
        }),
        Some(("list", _matches)) => Ok(CliCommand::SecretList {}),
        Some(("create", matches)) => Ok(CliCommand::SecretCreate {
            name: parse_string("name", matches)?,
            word: parse_string("word", matches)?,
        }),
        Some(("revoke", matches)) => Ok(CliCommand::SecretRevoke {
            name: parse_string("name", matches)?,
            delegate: parse_pubkey("delegate", matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_thread_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("crate-info", _)) => Ok(CliCommand::ThreadCrateInfo {}),
        Some(("create", matches)) => Ok(CliCommand::ThreadCreate {
            id: parse_string("id", matches)?,
            kickoff_instruction: parse_instruction_file("kickoff_instruction", matches)?,
            trigger: parse_trigger(matches)?,
        }),
        Some(("delete", matches)) => Ok(CliCommand::ThreadDelete {
            id: parse_string("id", matches)?,
        }),
        Some(("get", matches)) => Ok(CliCommand::ThreadGet {
            id: parse_string("id", matches).ok(),
            address: parse_pubkey("address", matches).ok(),
        }),
        Some(("pause", matches)) => Ok(CliCommand::ThreadPause {
            id: parse_string("id", matches)?,
        }),
        Some(("resume", matches)) => Ok(CliCommand::ThreadResume {
            id: parse_string("id", matches)?,
        }),
        Some(("reset", matches)) => Ok(CliCommand::ThreadReset {
            id: parse_string("id", matches)?,
        }),
        Some(("update", matches)) => Ok(CliCommand::ThreadUpdate {
            id: parse_string("id", matches)?,
            rate_limit: parse_u64("rate_limit", matches).ok(),
            schedule: parse_string("schedule", matches).ok(),
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_registry_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("get", _)) => Ok(CliCommand::RegistryGet {}),
        Some(("unlock", _)) => Ok(CliCommand::RegistryUnlock {}),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_webhook_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("get", matches)) => Ok(CliCommand::WebhookGet {
            id: parse_string("id", matches)?.into_bytes(),
        }),
        Some(("create", matches)) => Ok(CliCommand::WebhookCreate {
            body: parse_string("body", matches)?.into_bytes(),
            id: parse_string("id", matches)?.into_bytes(),
            method: parse_http_method("method", matches)?,
            url: parse_string("url", matches)?,
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

fn parse_worker_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("create", matches)) => Ok(CliCommand::WorkerCreate {
            signatory: parse_keypair_file("signatory_keypair", matches)?,
        }),
        Some(("get", matches)) => Ok(CliCommand::WorkerGet {
            id: parse_u64("id", matches)?,
        }),
        Some(("update", matches)) => Ok(CliCommand::WorkerUpdate {
            id: parse_u64("id", matches)?,
            signatory: parse_keypair_file("signatory_keypair", matches).ok(),
        }),
        _ => Err(CliError::CommandNotRecognized(
            matches.subcommand().unwrap().0.into(),
        )),
    }
}

// Arg parsers

fn parse_trigger(matches: &ArgMatches) -> Result<Trigger, CliError> {
    if matches.is_present("account") {
        return Ok(Trigger::Account {
            address: parse_pubkey("address", matches)?,
            offset: 0, // TODO
            size: 32,  // TODO
        });
    } else if matches.is_present("cron") {
        return Ok(Trigger::Cron {
            schedule: parse_string("cron", matches)?,
            skippable: true,
        });
    } else if matches.is_present("now") {
        return Ok(Trigger::Now);
    }

    Err(CliError::BadParameter("trigger".into()))
}

fn parse_instruction_file(
    arg: &str,
    matches: &ArgMatches,
) -> Result<SerializableInstruction, CliError> {
    let filepath = parse_string(arg, matches)?;
    let text = fs::read_to_string(filepath).map_err(|_err| CliError::BadParameter(arg.into()))?;
    let ix: JsonInstructionData =
        serde_json::from_str(text.as_str()).expect("JSON was not well-formatted");
    SerializableInstruction::try_from(&ix)
}

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

pub fn parse_usize(arg: &str, matches: &ArgMatches) -> Result<usize, CliError> {
    Ok(parse_string(arg, matches)?
        .parse::<usize>()
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

impl TryFrom<&JsonInstructionData> for SerializableInstruction {
    type Error = CliError;

    fn try_from(value: &JsonInstructionData) -> Result<Self, Self::Error> {
        Ok(SerializableInstruction {
            program_id: Pubkey::from_str(value.program_id.as_str())
                .map_err(|_err| CliError::BadParameter("Could not parse pubkey".into()))?,
            accounts: value
                .accounts
                .iter()
                .map(|acc| SerializableAccount::try_from(acc).unwrap())
                .collect::<Vec<SerializableAccount>>(),
            data: value.data.clone(),
        })
    }
}

// pub fn _parse_instruction(filepath: &String) -> Result<Instruction, CliError> {
//     let text =
//         fs::read_to_string(filepath).map_err(|_err| CliError::BadParameter("filepath".into()))?;
//     let ix: JsonInstructionData =
//         serde_json::from_str(text.as_str()).expect("JSON was not well-formatted");
//     Instruction::try_from(&ix)
// }

#[derive(Debug, JsonDeserialize, JsonSerialize, PartialEq)]
pub struct JsonAccountMetaData {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl TryFrom<&JsonAccountMetaData> for SerializableAccount {
    type Error = CliError;

    fn try_from(value: &JsonAccountMetaData) -> Result<Self, Self::Error> {
        Ok(SerializableAccount {
            pubkey: Pubkey::from_str(value.pubkey.as_str())
                .map_err(|_err| CliError::BadParameter("Could not parse pubkey".into()))?,
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
