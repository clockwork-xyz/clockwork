use anchor_lang::prelude::*;

pub const SEED_DAEMON: &[u8] = b"daemon";

#[account]
pub struct Daemon {
    pub owner: Pubkey,
    pub bump: u8,
}
