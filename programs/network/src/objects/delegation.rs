use anchor_lang::{prelude::*, AnchorDeserialize};

pub const SEED_DELEGATION: &[u8] = b"delegation";

/// An account to manage a token holder's stake delegation with a particiular a worker.
#[account]
#[derive(Debug)]
pub struct Delegation {
    /// The claimable yield paid out by the worker.
    pub claimable_yield: u64,

    /// The number of tokens that have been staked with this worker.
    pub stake_amount: u64,

    /// The worker the stake has been delegated to.
    pub worker: Pubkey,
}

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
