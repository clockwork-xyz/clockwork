use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;

pub const SEED_FEE: &[u8] = b"fee";

#[account]
#[derive(Debug)]
pub struct Fee {
    pub daemon: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

impl From<Vec<u8>> for Fee {
    fn from(data: Vec<u8>) -> Self {
        Fee::try_deserialize(&mut data.as_slice()).unwrap()
    }
}
