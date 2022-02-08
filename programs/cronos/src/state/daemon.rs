use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;

pub const SEED_DAEMON: &[u8] = b"daemon";

#[account]
#[derive(Debug)]
pub struct Daemon {
    pub owner: Pubkey,
    pub task_count: u128,
    pub bump: u8,
}

impl From<Vec<u8>> for Daemon {
    fn from(data: Vec<u8>) -> Self {
        Daemon::try_deserialize(&mut data.as_slice()).unwrap()
    }
}
