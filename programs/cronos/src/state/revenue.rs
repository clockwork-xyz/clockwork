use anchor_lang::prelude::*;

pub const SEED_REVENUE: &[u8] = b"revenue";

#[account]
#[derive(Debug)]
pub struct Revenue {
    pub daemon: Pubkey,
    pub balance: u64,
    pub bump: u8,
}
