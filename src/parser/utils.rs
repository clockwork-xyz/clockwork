use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use clap::ArgMatches;

use crate::error::CliError;

pub fn parse_i64(arg: &String, matches: &ArgMatches) -> Result<i64, CliError> {
    matches
        .value_of(arg)
        .ok_or(CliError::BadParameter(arg.into()))?
        .parse::<i64>()
        .map_err(|_err| CliError::BadParameter(arg.into()))
}

pub fn parse_pubkey(arg: &String, matches: &ArgMatches) -> Result<Pubkey, CliError> {
    let address = matches
        .value_of(arg)
        .ok_or(CliError::BadParameter(arg.into()))?;
    Ok(Pubkey::from_str(address).map_err(|_err| CliError::BadParameter(arg.into()))?)
}
