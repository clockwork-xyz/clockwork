use anchor_lang::prelude::*;

pub const SEED_CONFIG: &[u8] = b"config";

#[account]
#[derive(Debug)]
pub struct Config {
    pub admin_authority: Pubkey,
    pub frame_interval: u64,
    pub program_fee: u64,
    pub worker_fee: u64,
    pub bump: u8,
}
