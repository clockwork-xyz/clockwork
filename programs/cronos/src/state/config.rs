use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use std::convert::TryFrom;

pub const SEED_CONFIG: &[u8] = b"config";

#[account]
#[derive(Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub program_fee: u64,
    pub worker_fee: u64,
    pub bump: u8,
}

impl TryFrom<Vec<u8>> for Config {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Config::try_deserialize(&mut data.as_slice())
    }
}

impl Config {
    pub fn find_pda() -> PDA {
        Pubkey::find_program_address(&[SEED_CONFIG], &crate::ID)
    }
}
