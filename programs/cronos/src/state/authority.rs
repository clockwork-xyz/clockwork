use anchor_lang::prelude::*;

pub const SEED_AUTHORITY: &[u8] = b"authority";

#[account]
#[derive(Debug)]
pub struct Authority {
    pub bump: u8,
}
