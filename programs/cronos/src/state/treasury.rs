use anchor_lang::prelude::*;

pub const SEED_TREASURY: &[u8] = b"treasury";

#[account]
#[derive(Debug)]
pub struct Treasury {
    pub bump: u8,
}
