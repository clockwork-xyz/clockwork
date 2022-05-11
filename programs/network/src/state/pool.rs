use {
    super::{Config, Snapshot},
    crate::{
        errors::CronosError,
        pda::PDA,
        state::{SnapshotEntry, SnapshotStatus},
    },
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::{
        collections::{hash_map::DefaultHasher, VecDeque},
        convert::TryFrom,
        hash::{Hash, Hasher},
    },
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

    fn cycle(
        &mut self,
        config: &Account<Config>,
        snapshot: &Account<Snapshot>,
        snapshot_entry: &Account<SnapshotEntry>,
    ) -> Result<()>;
}

impl PoolAccount for Account<'_, Pool> {
    fn new(&mut self) -> Result<()> {
        self.delegates = VecDeque::new();
        Ok(())
    }

    fn cycle(
        &mut self,
        config: &Account<Config>,
        snapshot: &Account<Snapshot>,
        snapshot_entry: &Account<SnapshotEntry>,
    ) -> Result<()> {
        // Verify the snapshot is current
        require!(
            snapshot.status == SnapshotStatus::Current,
            CronosError::SnapshotNotCurrent
        );

        // Sample the nonce value
        let sample: u64 = self.nonce.checked_rem(snapshot.stake_amount_total).unwrap();

        // Verify the sample is within the snapshot entry's stake range
        require!(
            sample >= snapshot_entry.stake_offset
                && sample
                    < snapshot_entry
                        .stake_offset
                        .checked_add(snapshot_entry.stake_amount)
                        .unwrap(),
            CronosError::InvalidSnapshotEntry
        );

        // Pop a delegate out of the pool
        self.delegates.pop_front();

        // Push sampled node into the pool
        self.delegates.push_back(snapshot_entry.node_identity);

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
