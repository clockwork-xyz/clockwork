use anchor_lang::prelude::*;

pub const SEED_DAEMON: &[u8] = b"daemon";

#[account]
#[derive(Debug)]
pub struct Daemon {
    pub owner: Pubkey,
    pub task_count: u128,
    pub bump: u8,
}
