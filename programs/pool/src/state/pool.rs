use super::{Config, SnapshotPage, Snapshot};

use {
    crate::pda::PDA,
    anchor_lang::{AnchorDeserialize, prelude::*},
    std::convert::TryFrom,
};

pub const SEED_POOL: &[u8] = b"pool";

/**
 * Pool
 */

#[account]
#[derive(Debug)]
pub struct Pool {
    pub bump: u8,
    pub delegates: Vec<Pubkey>,
    pub nonce: Pubkey,
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
    fn new(&mut self, bump: u8) -> Result<()>;

    fn cycle(
        &mut self, 
        config: &Account<Config>, 
        snapshot: &Account<Snapshot>,
        snapshot_page: &Account<SnapshotPage>
    ) -> Result<()>;
}

impl PoolAccount for Account<'_, Pool> {
    fn new(&mut self, bump: u8) -> Result<()> {
        self.bump = bump;
        self.delegates = vec![];
        Ok(())
    }

    fn cycle(
        &mut self, 
        config: &Account<Config>,
        snapshot: &Account<Snapshot>,
        snapshot_page: &Account<SnapshotPage>
    ) -> Result<()> {

        // TODO Sample the nonce value
        // TODO Verify the sample is withing the snapshot page range 

        // Pop the last delegate out of the pool
        self.delegates.pop();

        // TODO Push the new delegate into the pool

        // Drain pool to configured size limit
        while self.delegates.len() > config.pool_size {
            self.delegates.pop();
        }

        Ok(())
    }
}

