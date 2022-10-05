use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_SNAPSHOT_ENTRY: &[u8] = b"snapshot_entry";

/**
 * SnapshotEntry
 */
#[account]
#[derive(Debug)]
pub struct SnapshotEntry {
    pub id: u64,
    pub snapshot: Pubkey,
    pub stake_amount: u64,
    pub stake_offset: u64,
    pub worker: Pubkey,
}

impl SnapshotEntry {
    pub fn pubkey(snapshot: Pubkey, id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[
                SEED_SNAPSHOT_ENTRY,
                snapshot.as_ref(),
                id.to_be_bytes().as_ref(),
            ],
            &crate::ID,
        )
        .0
    }
}

impl TryFrom<Vec<u8>> for SnapshotEntry {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        SnapshotEntry::try_deserialize(&mut data.as_slice())
    }
}

/**
 * SnapshotEntryAccount
 */

pub trait SnapshotEntryAccount {
    fn pubkey(&self) -> Pubkey;

    fn init(
        &mut self,
        id: u64,
        snapshot: Pubkey,
        stake_offset: u64,
        stake_amount: u64,
        worker: Pubkey,
    ) -> Result<()>;
}

impl SnapshotEntryAccount for Account<'_, SnapshotEntry> {
    fn pubkey(&self) -> Pubkey {
        SnapshotEntry::pubkey(self.snapshot, self.id)
    }

    fn init(
        &mut self,
        id: u64,
        snapshot: Pubkey,
        stake_offset: u64,
        stake_amount: u64,
        worker: Pubkey,
    ) -> Result<()> {
        self.id = id;
        self.snapshot = snapshot;
        self.stake_offset = stake_offset;
        self.stake_amount = stake_amount;
        self.worker = worker;
        Ok(())
    }
}
