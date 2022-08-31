use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::{collections::VecDeque, convert::TryFrom},
};

pub const SEED_POOL: &[u8] = b"pool";

/**
 * Pool
 */

#[account]
#[derive(Debug)]
pub struct Pool {
    pub name: String,
    pub size: usize,
    pub workers: VecDeque<Pubkey>,
}

impl Pool {
    pub fn pubkey(name: String) -> Pubkey {
        Pubkey::find_program_address(&[SEED_POOL, name.as_bytes()], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Pool {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Pool::try_deserialize(&mut data.as_slice())
    }
}

/**
 * PoolSettings
 */

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PoolSettings {
    pub size: usize,
}

/**
 * PoolAccount
 */

pub trait PoolAccount {
    fn init(&mut self, name: String, size: usize) -> Result<()>;

    fn rotate(&mut self, worker: Pubkey) -> Result<()>;

    fn update(&mut self, settings: &PoolSettings) -> Result<()>;
}

impl PoolAccount for Account<'_, Pool> {
    fn init(&mut self, name: String, size: usize) -> Result<()> {
        self.name = name;
        self.size = size;
        self.workers = VecDeque::new();
        Ok(())
    }

    fn rotate(&mut self, worker: Pubkey) -> Result<()> {
        // Pop a worker out of the pool
        self.workers.pop_front();

        // Push provided worker into the pool
        self.workers.push_back(worker);

        // Drain pool to the configured size limit
        while self.workers.len() > self.size {
            self.workers.pop_front();
        }

        Ok(())
    }

    fn update(&mut self, settings: &PoolSettings) -> Result<()> {
        self.size = settings.size;
        Ok(())
    }
}
