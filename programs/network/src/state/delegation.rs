use anchor_lang::{prelude::*, AnchorDeserialize};

pub const SEED_DELEGATION: &[u8] = b"delegation";

/// An account to manage a token holder's stake delegation with a particiular a worker.
#[account]
#[derive(Debug)]
pub struct Delegation {
    /// The authority of this delegation account.
    pub authority: Pubkey,

    /// The id of this delegation (auto-incrementing integer relative to worker)
    pub id: u64,

    /// The number of delegated tokens currently locked with the worker.
    pub stake_amount: u64,

    /// The worker to delegate stake to.
    pub worker: Pubkey,

    /// The number of lamports claimable as yield by the authority.
    pub yield_balance: u64,
}

impl Delegation {
    pub fn pubkey(worker: Pubkey, id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_DELEGATION, worker.as_ref(), id.to_be_bytes().as_ref()],
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

/// DelegationAccount
pub trait DelegationAccount {
    fn pubkey(&self) -> Pubkey;

    fn init(&mut self, authority: Pubkey, id: u64, worker: Pubkey) -> Result<()>;
}

impl DelegationAccount for Account<'_, Delegation> {
    fn pubkey(&self) -> Pubkey {
        Delegation::pubkey(self.worker, self.id)
    }

    fn init(&mut self, authority: Pubkey, id: u64, worker: Pubkey) -> Result<()> {
        self.authority = authority;
        self.id = id;
        self.stake_amount = 0;
        self.worker = worker;
        self.yield_balance = 0;
        Ok(())
    }
}
