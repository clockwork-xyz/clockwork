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
    pub entries: Vec<(Pubkey, u64)>,
    pub id: u128,
}

impl SnapshotPage {
    pub fn pda(id: u128) -> PDA {
        Pubkey::find_program_address(
            &[
                SEED_SNAPSHOT_PAGE, 
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
    fn new(&mut self, bump: u8, id: u128) -> Result<()>;
}

impl SnapshotPageAccount for Account<'_, SnapshotPage> {
    fn new(&mut self, bump: u8, id: u128) -> Result<()> {
        self.bump = bump;
        self.entries = vec![];
        self.id = id;
        Ok(())
    }
}

