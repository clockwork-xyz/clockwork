use {
    crate::{errors::SnapshotError, pda::PDA},
    super::{Node, SnapshotPage, Registry, RegistryPage, RegistryAccount},
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
    pub node_count: u64,
    pub page_count: u64,
    pub status: SnapshotStatus,
    pub cumulative_stake: u64,
    pub ts: i64,
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
    fn start(
        &mut self, 
        bump: u8,
        registry: &mut Account<Registry>, 
        timestamp: i64
    ) -> Result<()>;

    fn capture(
        &mut self, 
        nodes: Vec<&Account<Node>>,
        registry: &mut Account<Registry>, 
        registry_page: &Account<RegistryPage>,
        snapshot_page: &mut Account<SnapshotPage>, 
    ) -> Result<()>;
}

impl SnapshotAccount for Account<'_, Snapshot> {
    fn start(
        &mut self, 
        bump: u8, 
        registry: &mut Account<Registry>, 
        ts: i64
    ) -> Result<()> {
        self.bump = bump;
        self.node_count = 0;
        self.page_count = 0;
        self.status = SnapshotStatus::InProgress { page_id: 0 };
        self.ts = ts;

        registry.lock()
    }

    fn capture(
        &mut self, 
        nodes: Vec<&Account<Node>>,
        registry: &mut Account<Registry>, 
        registry_page: &Account<RegistryPage>,
        snapshot_page: &mut Account<SnapshotPage>
    ) -> Result<()> {

        // Validate the snapshot state
        match self.clone().into_inner().status {
            SnapshotStatus::Done => {
                return Err(SnapshotError::AlreadyDone.into());
            },
            SnapshotStatus::InProgress { 
                page_id
            } => {
                require!(
                    registry_page.id == page_id, 
                    SnapshotError::InvalidRegistryPage
                );
            }
        };
        
        // Record the cumulative stake
        for node in nodes {
            self.cumulative_stake = self.cumulative_stake.checked_add(node.stake).unwrap();
            snapshot_page.entries.push((node.authority, self.cumulative_stake));
        }

        // If all pages have been processed, mark the snapshot as done and unlock the registry
        if registry_page.id.checked_add(1).unwrap() >= registry.page_count {
            self.status = SnapshotStatus::Done;
            registry.unlock(self.ts).unwrap();
        }
        
        Ok(())
    }
}

/**
 * SnapshotStatus
 */
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub enum SnapshotStatus {
    Done,
    InProgress {
        page_id: u64,
    }
}
