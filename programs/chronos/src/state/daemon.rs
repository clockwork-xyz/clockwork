use anchor_lang::prelude::*;

pub const SEED_DAEMON: &[u8] = b"daemon";

#[account]
pub struct Daemon {
    pub owner: Pubkey,
    pub total_task_count: u128,
    pub executed_task_count: u128,
    pub bump: u8,
}
