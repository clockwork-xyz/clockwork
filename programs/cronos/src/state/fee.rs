use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use std::convert::TryFrom;

pub const SEED_FEE: &[u8] = b"fee";

#[account]
#[derive(Debug)]
pub struct Fee {
    pub daemon: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

impl TryFrom<Vec<u8>> for Fee {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Fee::try_deserialize(&mut data.as_slice())
    }
}
