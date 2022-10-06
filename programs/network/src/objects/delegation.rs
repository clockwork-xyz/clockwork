use anchor_lang::{prelude::*, AnchorDeserialize};

pub const SEED_DELEGATION: &[u8] = b"delegation";

/// An account to manage a token holder's stake delegation with a particiular a worker.
#[account]
#[derive(Debug)]
pub struct Delegation {
    /// The authority of this stake delegation account.
    pub authority: Pubkey,

    /// The number of tokens the authority has delegated to this worker.
    pub stake_balance: u64,

    /// The worker the stake has been delegated to.
    pub worker: Pubkey,

    /// The amount claimable lamports payable as yield to the authority.
    pub yield_balance: u64,
}

impl Delegation {
    pub fn pubkey(authority: Pubkey, worker: Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_DELEGATION, authority.as_ref(), worker.as_ref()],
            &crate::ID,
        )
        .0
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
