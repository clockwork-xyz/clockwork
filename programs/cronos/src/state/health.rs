use anchor_lang::prelude::*;

pub const SEED_HEALTH: &[u8] = b"health";

#[account]
#[derive(Debug)]
pub struct Health {
    pub real_time: u64,
    pub target_time: u64,
    pub bump: u8,
}
