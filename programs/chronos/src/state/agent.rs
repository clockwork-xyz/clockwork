use anchor_lang::prelude::*;

pub const SEED_AGENT: &[u8] = b"agent";

#[account]
pub struct Agent {
    pub owner: Pubkey,
    pub bump: u8,
}
