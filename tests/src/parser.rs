use {
    crate::cli::{CliCommand, CliError},
    clap::ArgMatches,
    serde::{Deserialize as JsonDeserialize, Serialize as JsonSerialize},
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    std::{convert::TryFrom, str::FromStr},
};

impl TryFrom<&ArgMatches> for CliCommand {
    type Error = CliError;

    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        Ok(CliCommand::Benchmark {
            count: parse_u32("count", matches)?,
            parallelism: parse_f32("parallelism", matches)?,
            recurrence: parse_u32("recurrence", matches)?,
        })
    }
}

// Arg parsers

pub fn parse_f32(arg: &str, matches: &ArgMatches) -> Result<f32, CliError> {
    let value = parse_string(arg, matches)?
        .parse::<f32>()
        .map_err(|_err| CliError::BadParameter(arg.into()))
        .unwrap();

    Ok(value)
}

pub fn parse_u32(arg: &str, matches: &ArgMatches) -> Result<u32, CliError> {
    let value = parse_string(arg, matches)?
        .parse::<u32>()
        .map_err(|_err| CliError::BadParameter(arg.into()))
        .unwrap();

    Ok(value)
}

fn parse_string(arg: &str, matches: &ArgMatches) -> Result<String, CliError> {
    Ok(matches
        .value_of(arg)
        .ok_or(CliError::BadParameter(arg.into()))?
        .to_string())
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
                .map_err(|_err| CliError::BadParameter("Pubkey not parsable".into()))?,
            is_signer: value.is_signer,
            is_writable: value.is_writable,
        })
    }
}
