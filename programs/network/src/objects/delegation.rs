use anchor_lang::{prelude::*, AnchorDeserialize};

pub const SEED_DELEGATION: &[u8] = b"delegation";

/**
 * Delegation
 */

#[account]
#[derive(Debug)]
pub struct Delegation {}

impl Delegation {
    pub fn pubkey() -> Pubkey {
        Pubkey::find_program_address(&[SEED_DELEGATION], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Delegation {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Delegation::try_deserialize(&mut data.as_slice())
    }
}

/**
 * DelegationAccount
 */

pub trait DelegationAccount {}

impl DelegationAccount for Account<'_, Delegation> {}
