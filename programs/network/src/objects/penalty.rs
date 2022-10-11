use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_PENALTY: &[u8] = b"penalty";

/// Escrows the lamport balance owed to a particular worker.
#[account]
#[derive(Debug)]
pub struct Penalty {
    /// The worker who was penalized.
    pub worker: Pubkey,
}

impl Penalty {
    /// Derive the pubkey of a fee account.
    pub fn pubkey(worker: Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[SEED_PENALTY, worker.as_ref()], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Penalty {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Penalty::try_deserialize(&mut data.as_slice())
    }
}

/// Trait for reading and writing to a penalty account.
pub trait PenaltyAccount {
    /// Get the pubkey of the penalty account.
    fn pubkey(&self) -> Pubkey;

    /// Initialize the account to hold penalty object.
    fn init(&mut self, worker: Pubkey) -> Result<()>;
}

impl PenaltyAccount for Account<'_, Penalty> {
    fn pubkey(&self) -> Pubkey {
        Penalty::pubkey(self.worker)
    }

    fn init(&mut self, worker: Pubkey) -> Result<()> {
        self.worker = worker;
        Ok(())
    }
}
