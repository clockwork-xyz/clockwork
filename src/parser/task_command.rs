use std::fs;
use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;
use cronos_sdk::account::{AccountMetaData, InstructionData};
use serde::{Deserialize as JsonDeserialize, Serialize as JsonSerialize};

use crate::{command::CliCommand, error::CliError};

use super::utils::parse_pubkey;

pub fn task_command(matches: &ArgMatches) -> Result<CliCommand, CliError> {
    match matches.subcommand() {
        Some(("new", matches)) => {
            // TODO parse arguments
            let filepath = String::from_str(
                matches
                    .value_of("filepath")
                    .ok_or(CliError::BadParameter("filepath".into()))?,
            )
            .map_err(|_err| CliError::BadParameter("filepath".into()))?;

            println!("Filepath: {}", filepath);

            let ix = parse_instruction_data(&filepath)?;
            println!("Instruction! {}", ix);

            let exec_at = None;
            let stop_at = None;
            let recurr = None;
            Ok(CliCommand::TaskNew {
                exec_at,
                stop_at,
                recurr,
            })
        }
        _ => Ok(CliCommand::TaskData {
            address: parse_pubkey(&"address".into(), matches)?,
        }),
    }
}

#[derive(Debug, JsonDeserialize, JsonSerialize)]
pub struct JsonInstructionData {
    /// Pubkey of the instruction processor that executes this instruction
    pub program_id: String,
    /// Metadata for what accounts should be passed to the instruction processor
    pub accounts: Vec<JsonAccountMetaData>,
    /// Opaque data passed to the instruction processor
    pub data: Vec<u8>,
}

impl TryFrom<&JsonInstructionData> for InstructionData {
    type Error = CliError;

    fn try_from(value: &JsonInstructionData) -> Result<Self, Self::Error> {
        Ok(InstructionData {
            program_id: Pubkey::from_str(value.program_id.as_str())
                .map_err(|_err| CliError::BadParameter("asdf".into()))?,
            accounts: value
                .accounts
                .iter()
                .map(|ix| AccountMetaData::try_from(ix).unwrap())
                .collect::<Vec<AccountMetaData>>(),
            data: value.data.clone(),
        })
    }
}

#[derive(Debug, JsonDeserialize, JsonSerialize, PartialEq)]
pub struct JsonAccountMetaData {
    /// An account's public key
    pub pubkey: String,
    /// True if an Instruction requires a Transaction signature matching `pubkey`.
    pub is_signer: bool,
    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,
}

impl TryFrom<&JsonAccountMetaData> for AccountMetaData {
    type Error = CliError;

    fn try_from(value: &JsonAccountMetaData) -> Result<Self, Self::Error> {
        Ok(AccountMetaData {
            pubkey: Pubkey::from_str(value.pubkey.as_str())
                .map_err(|_err| CliError::BadParameter("asdf".into()))?,
            is_signer: value.is_signer,
            is_writable: value.is_writable,
        })
    }
}

fn parse_instruction_data(filepath: &String) -> Result<InstructionData, CliError> {
    // let mut file = File::open("text.json").unwrap();
    let text =
        fs::read_to_string(filepath).map_err(|_err| CliError::BadParameter("filepath".into()))?;

    let ix: JsonInstructionData =
        serde_json::from_str(text.as_str()).expect("JSON was not well-formatted");

    println!("JSON: {:?}", ix);

    InstructionData::try_from(&ix)

    // Ok(InstructionData {})
    // Err(CliError::BadParameter("IDK".into()))
    // Ok(ix)
}
