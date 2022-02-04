use anchor_lang::prelude::*;

pub const SEED_AUTHORITY: &[u8] = b"authority";

#[account]
pub struct Authority {
    pub bump: u8,
}
