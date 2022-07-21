use {
    super::{Node, SnapshotEntry},
    crate::state::SnapshotEntryAccount,
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
    pub id: u64,
    pub node_count: u64,
    pub stake_total: u64,
    pub status: SnapshotStatus,
}

impl Snapshot {
    pub fn pubkey(id: u64) -> Pubkey {
        Pubkey::find_program_address(&[SEED_SNAPSHOT, id.to_be_bytes().as_ref()], &crate::ID).0
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

    fn capture(
        &mut self,
        entry: &mut Account<SnapshotEntry>,
        node: &Account<Node>,
        stake: &Account<TokenAccount>,
    ) -> Result<()>;
}

impl SnapshotAccount for Account<'_, Snapshot> {
    fn new(&mut self, id: u64) -> Result<()> {
        self.id = id;
        self.node_count = 0;
        self.status = SnapshotStatus::InProgress;
        Ok(())
    }

    fn capture(
        &mut self,
        entry: &mut Account<SnapshotEntry>,
        node: &Account<Node>,
        stake: &Account<TokenAccount>,
    ) -> Result<()> {
        // Record the new snapshot entry
        entry.new(
            self.node_count,
            self.key(),
            self.stake_total,
            stake.amount,
            node.worker,
        )?;

        // Update the snapshot's entry count
        self.node_count = self.node_count.checked_add(1).unwrap();

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
    Archived,
    Closing,
    Current,
    InProgress,
}
