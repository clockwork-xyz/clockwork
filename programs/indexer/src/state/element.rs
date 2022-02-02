use anchor_lang::prelude::*;

/// Root seed for deriving Element account PDAs.
pub const SEED_ELEMENT: &[u8] = b"element";

/// Element accounts track an address's position in a list.
#[account]
pub struct Element {
    pub index: u128,
    pub value: Pubkey,
    pub bump: u8,
}
