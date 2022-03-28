use {
    crate::{errors::CronosError, pda::PDA, state::RegistryAccount},
    super::{Node, SnapshotEntry, SnapshotPage, Registry, RegistryPage},
    anchor_lang::{AnchorDeserialize, prelude::*},
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
    pub fn pda(ts: i64) -> PDA {
        Pubkey::find_program_address(&[
            SEED_SNAPSHOT,
            ts.to_be_bytes().as_ref(),
        ], &crate::ID)
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

    fn capture(
        &mut self, 
        nodes: Vec<&Account<Node>>,
        registry_page: &Account<RegistryPage>,
        snapshot_page: &mut Account<SnapshotPage>, 
    ) -> Result<()>;

    fn rotate(
        &mut self,
        clock: Sysvar<Clock>,
        next_snapshot: &mut Account<Snapshot>, 
        registry: &mut Account<Registry>
    ) -> Result<()>;
}

impl SnapshotAccount for Account<'_, Snapshot> {
    fn new(&mut self, bump: u8, id: u64) -> Result<()> {
        self.bump = bump;
        self.id = id;
        self.node_count = 0;
        self.page_count = 0;
        self.status = SnapshotStatus::InProgress { page_id: 0 };
        Ok(())
    }

    fn capture(
        &mut self, 
        nodes: Vec<&Account<Node>>,
        registry_page: &Account<RegistryPage>,
        snapshot_page: &mut Account<SnapshotPage>
    ) -> Result<()> {

        // Validate the snapshot status
        match self.clone().into_inner().status {
            SnapshotStatus::InProgress { page_id} => 
                require!(registry_page.id == page_id, CronosError::PageRangeInvalid),
            _ => return Err(CronosError::SnapshotNotInProgress.into())
        };
        
        // Record the cumulative stake of the node authorities
        for node in nodes {
            self.cumulative_stake = self.cumulative_stake.checked_add(node.stake).unwrap();
            snapshot_page.entries.push(SnapshotEntry {
                node_authority: node.authority,
                node_cumulative_stake: self.cumulative_stake
            });
        }
        
        Ok(())
    }

    fn rotate(&mut self, clock: Sysvar<Clock>, next_snapshot: &mut Account<Snapshot>, registry: &mut Account<Registry>) -> Result<()> {

        // Validate the snapshot is current and the registry is locked
        require!(self.status == SnapshotStatus::Current, CronosError::SnapshotNotCurrent);
        require!(registry.is_locked, CronosError::RegistryMustBeLocked);

        // Validate the next snapshot progress has captured the entire registry
        match next_snapshot.status {
            SnapshotStatus::InProgress { page_id } => 
                require!(page_id.checked_add(1).unwrap() >= registry.page_count, CronosError::SnapshotIncomplete),
            _ => return Err(CronosError::SnapshotNotInProgress.into())
        };        
        
        // Validate the next snapshot checksums
        require!(
            next_snapshot.page_count == registry.page_count && 
            next_snapshot.node_count == registry.node_count,
            CronosError::SnapshotIncomplete
        );

        // Mark the current snapshot as archived
        self.status = SnapshotStatus::Archived { ts: clock.unix_timestamp };

        // Mark the next snapshot as current
        next_snapshot.status = SnapshotStatus::Current;

        // Unlock the registry
        registry.unlock()
    }
}

/**
 * SnapshotStatus
 */
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub enum SnapshotStatus {
    Archived { ts: i64 },
    Current,
    InProgress { page_id: u64 }
}
