use {
    super::Snapshot,
    crate::{pda::PDA, state::SnapshotEntry},
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::{
        collections::hash_map::DefaultHasher,
        convert::TryFrom,
        hash::{Hash, Hasher},
    },
};

pub const SEED_CYCLER: &[u8] = b"cycler";

/**
 * Cycler
 */

#[account]
#[derive(Debug)]
pub struct Cycler {
    pub last_cycle_at: u64, // Slot of the last cycle
    pub nonce: u64,
}

impl Cycler {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_CYCLER], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Cycler {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Cycler::try_deserialize(&mut data.as_slice())
    }
}

/**
 * CyclerAccount
 */

pub trait CyclerAccount {
    fn new(&mut self) -> Result<()>;

    fn is_valid_entry(
        &mut self,
        entry: &Account<SnapshotEntry>,
        snapshot: &Account<Snapshot>,
    ) -> Result<bool>;

    fn hash_nonce(&mut self, slot: u64) -> Result<()>;
}

impl CyclerAccount for Account<'_, Cycler> {
    fn new(&mut self) -> Result<()> {
        // Start the nonce on a hash of the cycler's pubkey. This is an arbitrary value.
        let mut hasher = DefaultHasher::new();
        self.key().hash(&mut hasher);
        self.nonce = hasher.finish();
        self.last_cycle_at = 0;
        Ok(())
    }

    fn is_valid_entry(
        &mut self,
        entry: &Account<SnapshotEntry>,
        snapshot: &Account<Snapshot>,
    ) -> Result<bool> {
        // Return true if the sample is within the entry's stake range
        match self.nonce.checked_rem(snapshot.stake_total) {
            None => Ok(false),
            Some(sample) => Ok(sample >= entry.stake_offset
                && sample < entry.stake_offset.checked_add(entry.stake_amount).unwrap()),
        }
    }

    fn hash_nonce(&mut self, slot: u64) -> Result<()> {
        let mut hasher = DefaultHasher::new();
        self.nonce.hash(&mut hasher);
        self.nonce = hasher.finish();
        self.last_cycle_at = slot;
        Ok(())
    }
}
