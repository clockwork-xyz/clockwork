use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;

pub const SEED_CONFIG: &[u8] = b"config";

#[account]
#[derive(Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub program_fee: u64,
    pub worker_fee: u64,
    pub bump: u8,
}

impl From<Vec<u8>> for Config {
    fn from(data: Vec<u8>) -> Self {
        Config::try_deserialize(&mut data.as_slice()).unwrap()
    }
}
