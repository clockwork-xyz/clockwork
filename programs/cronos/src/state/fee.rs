use anchor_lang::prelude::*;

pub const SEED_FEE: &[u8] = b"fee";

#[account]
#[derive(Debug)]
pub struct Fee {
    pub daemon: Pubkey,
    pub balance: u64,
    pub bump: u8,
}
