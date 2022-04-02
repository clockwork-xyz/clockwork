use {
    crate::pda::PDA,
    anchor_lang::{AnchorDeserialize, prelude::*},
    std::convert::TryFrom,
};

pub const SEED_SNAPSHOT_PAGE: &[u8] = b"snapshot_page";

/**
 * SnapshotPage
 */

#[account]
#[derive(Debug)]
pub struct SnapshotPage {
    pub bump: u8,
    pub entries: Vec<SnapshotEntry>,
    pub id: u64,
}

impl SnapshotPage {

    pub fn pda(snapshot: Pubkey, id: u64) -> PDA {
        Pubkey::find_program_address(
            &[
                SEED_SNAPSHOT_PAGE, 
                snapshot.as_ref(),
                id.to_be_bytes().as_ref()
            ],
            &crate::ID
        )
    }
}

impl TryFrom<Vec<u8>> for SnapshotPage {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        SnapshotPage::try_deserialize(&mut data.as_slice())
    }
}

/**
 * SnapshotPageAccount
 */

pub trait SnapshotPageAccount {
    fn new(&mut self, bump: u8, id: u64) -> Result<()>;
}

impl SnapshotPageAccount for Account<'_, SnapshotPage> {
    fn new(&mut self, bump: u8, id: u64) -> Result<()> {
        self.bump = bump;
        self.entries = vec![];
        self.id = id;
        Ok(())
    }
}


/**
 * SnapshotEntry
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct SnapshotEntry {
    pub node_authority: Pubkey,
    pub node_cumulative_stake: u64
}