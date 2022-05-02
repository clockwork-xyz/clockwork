use {
    clap::ArgMatches,
    serde::{Deserialize as JsonDeserialize, Serialize as JsonSerialize},
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    std::str::FromStr,
};

use crate::error::TestError;

pub fn _parse_u32(arg: &String, matches: &ArgMatches) -> Result<u32, TestError> {
    let value = parse_string(arg, matches)?
        .parse::<u32>()
        .map_err(|_err| TestError::BadParameter(arg.into()))
        .unwrap();

    if arg == "time" && value > 59 {
        return Err(TestError::BadParameter(
            "time parameter cannot be greater than 59".to_string(),
        ));
    }

    Ok(value)
}

pub fn _parse_f32(arg: &String, matches: &ArgMatches) -> Result<f32, TestError> {
    parse_string(arg, matches)?
        .parse::<f32>()
        .map_err(|_err| TestError::BadParameter(arg.into()))
}

pub fn parse_string(arg: &String, matches: &ArgMatches) -> Result<String, TestError> {
    Ok(matches
        .value_of(arg)
        .ok_or(TestError::BadParameter(arg.into()))?
        .to_string())
}

#[derive(Debug, JsonDeserialize, JsonSerialize)]
pub struct JsonInstructionData {
    pub program_id: String,
    pub accounts: Vec<JsonAccountMetaData>,
    pub data: Vec<u8>,
}

impl TryFrom<&JsonInstructionData> for Instruction {
    type Error = TestError;

    fn try_from(value: &JsonInstructionData) -> Result<Self, Self::Error> {
        Ok(Instruction {
            program_id: Pubkey::from_str(value.program_id.as_str())
                .map_err(|_err| TestError::BadParameter("asdf".into()))?,
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
    type Error = TestError;

    fn try_from(value: &JsonAccountMetaData) -> Result<Self, Self::Error> {
        Ok(AccountMeta {
            pubkey: Pubkey::from_str(value.pubkey.as_str())
                .map_err(|_err| TestError::BadParameter("asdf".into()))?,
            is_signer: value.is_signer,
            is_writable: value.is_writable,
        })
    }
}
