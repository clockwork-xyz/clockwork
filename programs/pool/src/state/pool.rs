use {
    crate::{errors::{AccountError, PoolError, SnapshotError}, pda::PDA, state::SnapshotStatus},
    super::{Config, Registry, SnapshotPage, Snapshot},
    anchor_lang::{AnchorDeserialize, prelude::*},
    std::{collections::{hash_map::DefaultHasher, VecDeque}, convert::TryFrom, hash::{Hasher, Hash}},
};

pub const SEED_POOL: &[u8] = b"pool";

/**
 * Pool
 */

#[account]
#[derive(Debug)]
pub struct Pool {
    pub bump: u8,
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
    fn new(&mut self, bump: u8) -> Result<()>;

    fn cycle(
        &mut self, 
        config: &Account<Config>, 
        registry: &Account<Registry>,
        snapshot: &Account<Snapshot>,
        snapshot_page: &Account<SnapshotPage>
    ) -> Result<()>;
}

impl PoolAccount for Account<'_, Pool> {
    fn new(&mut self, bump: u8) -> Result<()> {
        require!(self.bump == 0, AccountError::AlreadyInitialized);
        self.bump = bump;
        self.delegates = VecDeque::new();
        Ok(())
    }

    fn cycle(
        &mut self, 
        config: &Account<Config>,
        registry: &Account<Registry>,
        snapshot: &Account<Snapshot>,
        snapshot_page: &Account<SnapshotPage>
    ) -> Result<()> {
        require!(snapshot.status == SnapshotStatus::Done, SnapshotError::NotDone);
        require!(registry.last_snapshot_ts.is_some(), PoolError::InvalidSnapshot);
        require!(registry.last_snapshot_ts.unwrap() == snapshot.ts, PoolError::InvalidSnapshot);
        require!(snapshot_page.entries.len() > 0, PoolError::InvalidSnapshotPage);

        // Sample the nonce value
        let sample = self.nonce.checked_rem(snapshot.cumulative_stake).unwrap();

        // Verify the sample is withing the snapshot page range 
        let r0: u64 = snapshot_page.entries.first().unwrap().1;
        let r1: u64 = snapshot_page.entries.last().unwrap().1;
        require!(sample >= r0 && sample <= r1, PoolError::InvalidSnapshotPage);

        // Pop the last delegate out of the pool
        self.delegates.pop_front();

        // Push the new delegate into the pool
        self.delegates.push_back(
            snapshot_page.entries.iter().rev().find(|e| e.1 <= sample).unwrap().0
        );

        // Drain pool to configured size limit
        while self.delegates.len() > config.pool_size {
            self.delegates.pop_front();
        }

        // Hash the nonce
        let mut hasher = DefaultHasher::new();
        self.nonce.hash(&mut hasher);
        self.nonce = hasher.finish();

        Ok(())
    }
}

