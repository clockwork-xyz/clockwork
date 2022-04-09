use {
    super::{Node, Registry, RegistryPage, SnapshotEntry, SnapshotPage},
    crate::{
        errors::CronosError,
        pda::PDA,
        state::{SnapshotPageAccount, PAGE_LIMIT},
    },
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
    pub id: u64,
    pub node_count: u64,
    pub page_count: u64,
    pub status: SnapshotStatus,
    pub cumulative_stake: u64,
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

    fn new_page(
        &mut self,
        page: &mut Account<SnapshotPage>,
        page_bump: u8,
        registry: &Account<Registry>,
    ) -> Result<()>;

    fn capture(
        &mut self,
        nodes: Vec<&Account<Node>>,
        registry_page: &Account<RegistryPage>,
        snapshot_page: &mut Account<SnapshotPage>,
    ) -> Result<()>;
}

impl SnapshotAccount for Account<'_, Snapshot> {
    fn new(&mut self, bump: u8, id: u64) -> Result<()> {
        self.bump = bump;
        self.id = id;
        self.node_count = 0;
        self.page_count = 0;
        self.status = SnapshotStatus::InProgress;
        Ok(())
    }

    fn new_page(
        &mut self,
        page: &mut Account<SnapshotPage>,
        page_bump: u8,
        registry: &Account<Registry>,
    ) -> Result<()> {
        // Validate the registry is locked
        require!(registry.is_locked, CronosError::RegistryMustBeLocked);

        // Validate the snapshot is in progress
        require!(
            self.status == SnapshotStatus::InProgress,
            CronosError::SnapshotNotInProgress
        );

        // Validate a new page is needed
        require!(
            self.page_count < registry.page_count,
            CronosError::PageRangeInvalid
        );

        // Initialize new page
        page.new(page_bump, registry.page_count)?;

        // Increment page count
        self.page_count = self.page_count.checked_add(1).unwrap();

        Ok(())
    }

    fn capture(
        &mut self,
        nodes: Vec<&Account<Node>>,
        registry_page: &Account<RegistryPage>,
        snapshot_page: &mut Account<SnapshotPage>,
    ) -> Result<()> {
        // Validate the snapshot is in progress
        require!(
            self.status == SnapshotStatus::InProgress,
            CronosError::SnapshotNotInProgress
        );

        // Validate this is the correct page to capture
        require!(
            snapshot_page.id == registry_page.id,
            CronosError::PageRangeInvalid
        );

        // Record the cumulative stake of the node authorities
        let offset = self.node_count.checked_div(PAGE_LIMIT as u64).unwrap() as usize;
        for i in 0..nodes.len() {
            let node = nodes[i];
            require!(
                registry_page.nodes[offset + i] == node.key(),
                CronosError::PageRangeInvalid,
            );
            self.cumulative_stake = self.cumulative_stake.checked_add(node.stake).unwrap();
            snapshot_page.entries.push(SnapshotEntry {
                node_authority: node.authority,
                node_cumulative_stake: self.cumulative_stake,
            });
        }

        // Update the snapshot's node count
        self.node_count = self.node_count.checked_add(nodes.len() as u64).unwrap();

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
