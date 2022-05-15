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
    pub delegates: VecDeque<Pubkey>,
    pub nonce: u64,
    pub snapshot_ts: i64,
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

    fn cycle(&mut self, config: &Account<Config>, delegate: &Pubkey) -> Result<()>;
}

impl PoolAccount for Account<'_, Pool> {
    fn new(&mut self) -> Result<()> {
        self.delegates = VecDeque::new();
        Ok(())
    }

    fn cycle(&mut self, config: &Account<Config>, delegate: &Pubkey) -> Result<()> {
        // Pop a delegate out of the pool
        self.delegates.pop_front();

        // Push provided delegate into the pool
        self.delegates.push_back(*delegate);

        // Drain pool to the configured size limit
        while self.delegates.len() > config.pool_size {
            self.delegates.pop_front();
        }

        Ok(())
    }
}
