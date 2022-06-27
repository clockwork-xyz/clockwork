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

pub const SEED_ROTATOR: &[u8] = b"rotator";

/**
 * Rotator
 */

#[account]
#[derive(Debug)]
pub struct Rotator {
    pub last_slot: u64, // Slot of the last cycle
    pub nonce: u64,
}

impl Rotator {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_ROTATOR], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Rotator {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Rotator::try_deserialize(&mut data.as_slice())
    }
}

/**
 * RotatorAccount
 */

pub trait RotatorAccount {
    fn new(&mut self) -> Result<()>;

    fn is_valid_entry(
        &mut self,
        entry: &Account<SnapshotEntry>,
        snapshot: &Account<Snapshot>,
    ) -> Result<bool>;

    fn hash_nonce(&mut self, slot: u64) -> Result<()>;
}

impl RotatorAccount for Account<'_, Rotator> {
    fn new(&mut self) -> Result<()> {
        // Start the nonce on a hash of the rotator's pubkey. This is an arbitrary value.
        let mut hasher = DefaultHasher::new();
        self.key().hash(&mut hasher);
        self.nonce = hasher.finish();
        self.last_slot = 0;
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
        // Hash the nonce
        let mut hasher = DefaultHasher::new();
        self.nonce.hash(&mut hasher);
        self.nonce = hasher.finish();

        // Record the slot value
        self.last_slot = slot;
        Ok(())
    }
}
