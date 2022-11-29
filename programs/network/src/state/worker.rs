use {
    crate::errors::*,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_WORKER: &[u8] = b"worker";

/// Worker
#[account]
#[derive(Debug)]
pub struct Worker {
    /// The worker's authority (owner).
    pub authority: Pubkey,
    /// The number of lamports claimable by the authority as commission for running the worker.
    pub commission_balance: u64,
    /// Integer between 0 and 100 determining the percentage of fees worker will keep as commission.
    pub commission_rate: u64,
    /// The worker's id.
    pub id: u64,
    /// The worker's signatory address (used to sign txs).
    pub signatory: Pubkey,
    /// The number delegations allocated to this worker.
    pub total_delegations: u64,
}

impl Worker {
    pub fn pubkey(id: u64) -> Pubkey {
        Pubkey::find_program_address(&[SEED_WORKER, id.to_be_bytes().as_ref()], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Worker {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Worker::try_deserialize(&mut data.as_slice())
    }
}

/// WorkerSettings
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WorkerSettings {
    pub commission_rate: u64,
    pub signatory: Pubkey,
}

/// WorkerAccount
pub trait WorkerAccount {
    fn pubkey(&self) -> Pubkey;

    fn init(&mut self, authority: &mut Signer, id: u64, signatory: &Signer) -> Result<()>;

    fn update(&mut self, settings: WorkerSettings) -> Result<()>;
}

impl WorkerAccount for Account<'_, Worker> {
    fn pubkey(&self) -> Pubkey {
        Worker::pubkey(self.id)
    }

    fn init(&mut self, authority: &mut Signer, id: u64, signatory: &Signer) -> Result<()> {
        self.authority = authority.key();
        self.commission_balance = 0;
        self.commission_rate = 0;
        self.id = id;
        self.signatory = signatory.key();
        self.total_delegations = 0;
        Ok(())
    }

    fn update(&mut self, settings: WorkerSettings) -> Result<()> {
        require!(
            settings.commission_rate.ge(&0) && settings.commission_rate.le(&100),
            ClockworkError::InvalidCommissionRate
        );
        self.commission_rate = settings.commission_rate;

        require!(
            settings.signatory.ne(&self.authority),
            ClockworkError::InvalidSignatory
        );
        self.signatory = settings.signatory;
        Ok(())
    }
}
