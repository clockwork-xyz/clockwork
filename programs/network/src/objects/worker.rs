use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::{collections::HashSet, convert::TryFrom},
};

pub const SEED_WORKER: &[u8] = b"worker";

/**
 * Worker
 */

#[account]
#[derive(Debug)]
pub struct Worker {
    pub authority: Pubkey,                // The worker's authority
    pub delegate: Pubkey,                 // The worker's delegate address (used to sign txs)
    pub id: u64,                          // The worker's id (auto-incrementing)
    pub supported_pools: HashSet<Pubkey>, // The set of pools this worker supports
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

/**
 * WorkerSettings
 */
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WorkerSettings {
    pub supported_pools: HashSet<Pubkey>,
}

/**
 * WorkerAccount
 */

pub trait WorkerAccount {
    fn pubkey(&self) -> Pubkey;

    fn init(&mut self, authority: &mut Signer, delegate: &Signer, id: u64) -> Result<()>;

    fn update(&mut self, settings: WorkerSettings) -> Result<()>;
}

impl WorkerAccount for Account<'_, Worker> {
    fn pubkey(&self) -> Pubkey {
        Worker::pubkey(self.id)
    }

    fn init(&mut self, authority: &mut Signer, delegate: &Signer, id: u64) -> Result<()> {
        self.authority = authority.key();
        self.delegate = delegate.key();
        self.id = id;
        Ok(())
    }

    fn update(&mut self, settings: WorkerSettings) -> Result<()> {
        self.supported_pools = settings.supported_pools;
        Ok(())
    }
}
