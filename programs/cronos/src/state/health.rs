use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;

pub const SEED_HEALTH: &[u8] = b"health";

#[account]
#[derive(Debug)]
pub struct Health {
    pub real_time: u64,
    pub target_time: u64,
    pub bump: u8,
}

impl From<Vec<u8>> for Health {
    fn from(data: Vec<u8>) -> Self {
        Health::try_deserialize(&mut data.as_slice()).unwrap()
    }
}
