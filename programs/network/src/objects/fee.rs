use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_FEE: &[u8] = b"fee";

/// Escrows the lamport balance owed to a particular worker.
#[account]
#[derive(Debug)]
pub struct Fee {
    /// The number of lamports that are distributable for this epoch period.
    pub distributable_balance: u64,
    /// The worker who received the fees.
    pub worker: Pubkey,
}

impl Fee {
    /// Derive the pubkey of a fee account.
    pub fn pubkey(worker: Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[SEED_FEE, worker.as_ref()], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Fee {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Fee::try_deserialize(&mut data.as_slice())
    }
}

/// Trait for reading and writing to a fee account.
pub trait FeeAccount {
    /// Get the pubkey of the fee account.
    fn pubkey(&self) -> Pubkey;

    /// Initialize the account to hold fee object.
    fn init(&mut self, worker: Pubkey) -> Result<()>;
}

impl FeeAccount for Account<'_, Fee> {
    fn pubkey(&self) -> Pubkey {
        Fee::pubkey(self.worker)
    }

    fn init(&mut self, worker: Pubkey) -> Result<()> {
        self.distributable_balance = 0;
        self.worker = worker;
        Ok(())
    }
}
