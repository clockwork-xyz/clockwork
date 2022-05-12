use {
    super::{Node, SnapshotEntry},
    crate::{errors::CronosError, pda::PDA, state::SnapshotEntryAccount},
    anchor_lang::{prelude::*, AnchorDeserialize},
    anchor_spl::token::TokenAccount,
    std::convert::TryFrom,
};

pub const SEED_SNAPSHOT: &[u8] = b"snapshot";

/**
 * Snapshot
 */

#[account]
#[derive(Debug)]
pub struct Snapshot {
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
    fn new(&mut self, id: u64) -> Result<()>;

    fn new_entry(
        &mut self,
        node: &Account<Node>,
        snapshot_entry: &mut Account<SnapshotEntry>,
        stake: &Account<TokenAccount>,
    ) -> Result<()>;
}

impl SnapshotAccount for Account<'_, Snapshot> {
    fn new(&mut self, id: u64) -> Result<()> {
        self.entry_count = 0;
        self.id = id;
        self.status = SnapshotStatus::InProgress;
        Ok(())
    }

    fn new_entry(
        &mut self,
        node: &Account<Node>,
        snapshot_entry: &mut Account<SnapshotEntry>,
        stake: &Account<TokenAccount>,
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

        // Validate the node is the owner of the stake account
        require!(stake.owner == node.key(), CronosError::InvalidStakeAccount);

        // Record the new snapshot entry
        snapshot_entry.new(
            self.entry_count,
            node.identity,
            self.stake_total,
            stake.amount,
            self.key(),
        )?;

        // Update the snapshot's entry count
        self.entry_count = self.entry_count.checked_add(1).unwrap();

        // Update the sum stake amount
        self.stake_total = self.stake_total.checked_add(stake.amount).unwrap();

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
