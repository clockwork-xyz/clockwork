use {
    crate::{errors::CronosError, pda::PDA, state::{SnapshotEntry, SnapshotStatus}},
    super::{Config, SnapshotPage, Snapshot},
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
        snapshot: &Account<Snapshot>,
        snapshot_page: &Account<SnapshotPage>
    ) -> Result<()>;
}

impl PoolAccount for Account<'_, Pool> {
    fn new(&mut self, bump: u8) -> Result<()> {
        require!(self.bump == 0, CronosError::AccountAlreadyInitialized);
        self.bump = bump;
        self.delegates = VecDeque::new();
        Ok(())
    }

    fn cycle(
        &mut self, 
        config: &Account<Config>,
        snapshot: &Account<Snapshot>,
        snapshot_page: &Account<SnapshotPage>
    ) -> Result<()> {
        require!(snapshot.status == SnapshotStatus::Current, CronosError::SnapshotNotCurrent);
        require!(snapshot_page.entries.len() > 0, CronosError::PageRangeInvalid);

        // Sample the nonce value
        let sample: u64 = self.nonce.checked_rem(snapshot.cumulative_stake).unwrap();

        // Verify the sample is within the snapshot page range 
        let first: &SnapshotEntry = snapshot_page.entries.first().unwrap();
        let last: &SnapshotEntry = snapshot_page.entries.last().unwrap();
        require!(
            sample >= first.node_cumulative_stake && 
            sample <= last.node_cumulative_stake, 
            CronosError::PageRangeInvalid
        );

        // Pop the last delegate out of the pool
        self.delegates.pop_front();

        // Push the sampled delegate into the pool
        self.delegates.push_back(
            snapshot_page.entries.iter().rev().find(
                |e| e.node_cumulative_stake <= sample
            )
            .unwrap()
            .node_authority
        );

        // Drain pool to the configured size limit
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

