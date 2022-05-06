use crate::state::SnapshotEntryAccount;

use super::{Node, SnapshotEntry};

use {
    crate::{errors::CronosError, pda::PDA},
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_SNAPSHOT: &[u8] = b"snapshot";

/**
 * Snapshot
 */

#[account]
#[derive(Debug)]
pub struct Snapshot {
    pub bump: u8,
    pub entry_count: u64,
    pub id: u64,
    pub stake_total: u64,
    pub status: SnapshotStatus,
}

impl Snapshot {
    pub fn pda(id: u64) -> PDA {
        Pubkey::find_program_address(&[SEED_SNAPSHOT, id.to_be_bytes().as_ref()], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Snapshot {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Snapshot::try_deserialize(&mut data.as_slice())
    }
}

/**
 * SnapshotAccount
 */

pub trait SnapshotAccount {
    fn new(&mut self, bump: u8, id: u64) -> Result<()>;

    fn new_entry(
        &mut self,
        node: &Account<Node>,
        snapshot_entry: &mut Account<SnapshotEntry>,
        snapshot_entry_bump: u8,
    ) -> Result<()>;
}

impl SnapshotAccount for Account<'_, Snapshot> {
    fn new(&mut self, bump: u8, id: u64) -> Result<()> {
        self.bump = bump;
        self.entry_count = 0;
        self.id = id;
        self.status = SnapshotStatus::InProgress;
        Ok(())
    }

    fn new_entry(
        &mut self,
        node: &Account<Node>,
        snapshot_entry: &mut Account<SnapshotEntry>,
        snapshot_entry_bump: u8,
    ) -> Result<()> {
        // Validate the snapshot is in progress
        require!(
            self.status == SnapshotStatus::InProgress,
            CronosError::SnapshotNotInProgress
        );

        // Validate this is the correct entry to capture
        require!(
            self.entry_count == snapshot_entry.id && node.id == snapshot_entry.id,
            CronosError::InvalidSnapshotEntry
        );

        // Record the new snapshot entry
        snapshot_entry.new(
            snapshot_entry_bump,
            self.entry_count,
            node.identity,
            self.stake_total,
            node.stake_size,
            self.key(),
        )?;

        // Update the snapshot's entry count
        self.entry_count = self.entry_count.checked_add(1).unwrap();

        Ok(())
    }
}

/**
 * SnapshotStatus
 */
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub enum SnapshotStatus {
    Archived { ts: i64 },
    Current,
    InProgress,
}
