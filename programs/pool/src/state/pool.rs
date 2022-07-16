use {
    super::Config,
    crate::pda::PDA,
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
    pub workers: VecDeque<Pubkey>,
}

impl Pool {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_POOL], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Pool {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Pool::try_deserialize(&mut data.as_slice())
    }
}

/**
 * PoolAccount
 */

pub trait PoolAccount {
    fn new(&mut self) -> Result<()>;

    fn rotate(&mut self, config: &Account<Config>, worker: Pubkey) -> Result<()>;
}

impl PoolAccount for Account<'_, Pool> {
    fn new(&mut self) -> Result<()> {
        self.workers = VecDeque::new();
        Ok(())
    }

    fn rotate(&mut self, config: &Account<Config>, worker: Pubkey) -> Result<()> {
        // Pop a worker out of the pool
        self.workers.pop_front();

        // Push provided worker into the pool
        self.workers.push_back(worker);

        // Drain pool to the configured size limit
        while self.workers.len() > config.pool_size {
            self.workers.pop_front();
        }

        Ok(())
    }
}
