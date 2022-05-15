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
    pub entry_id: u64,
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

    fn is_valid_delegate(
        &mut self,
        entry: &Account<SnapshotEntry>,
        snapshot: &Account<Snapshot>,
    ) -> Result<bool>;

    fn hash_nonce(&mut self) -> Result<()>;
}

impl CyclerAccount for Account<'_, Cycler> {
    fn new(&mut self) -> Result<()> {
        self.entry_id = 0;

        // Start the nonce on a hash of the cycler's pubkey. Note, this
        //  is an arbitrary value.
        let mut hasher = DefaultHasher::new();
        self.key().hash(&mut hasher);
        self.nonce = hasher.finish();

        Ok(())
    }

    fn is_valid_delegate(
        &mut self,
        entry: &Account<SnapshotEntry>,
        snapshot: &Account<Snapshot>,
    ) -> Result<bool> {
        // Return true if the sample is within the entry's stake range
        match self.nonce.checked_rem(snapshot.stake_total) {
            None => {
                msg!("Bad sample!");
                Ok(false)
            }
            Some(sample) => Ok(sample >= entry.stake_offset
                && sample < entry.stake_offset.checked_add(entry.stake_amount).unwrap()),
        }
    }

    fn hash_nonce(&mut self) -> Result<()> {
        let mut hasher = DefaultHasher::new();
        self.nonce.hash(&mut hasher);
        self.nonce = hasher.finish();
        Ok(())
    }
}
