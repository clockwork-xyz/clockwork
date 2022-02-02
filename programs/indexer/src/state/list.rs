use anchor_lang::prelude::*;

/// Root seed for deriving List account PDAs.
pub const SEED_LIST: &[u8] = b"lst";

/// List accounts store a list's metadata.
#[account]
pub struct List {
    pub owner: Pubkey,
    pub namespace: Pubkey,
    pub count: u128,
    pub bump: u8,
}
