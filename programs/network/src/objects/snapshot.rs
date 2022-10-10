use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_SNAPSHOT: &[u8] = b"snapshot";

/// Snapshot
#[account]
#[derive(Debug)]
pub struct Snapshot {
    pub id: u64,
    pub total_frames: u64,
    pub total_stake: u64,
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

/// SnapshotAccount
pub trait SnapshotAccount {
    fn pubkey(&self) -> Pubkey;

    fn init(&mut self, id: u64) -> Result<()>;
}

impl SnapshotAccount for Account<'_, Snapshot> {
    fn pubkey(&self) -> Pubkey {
        Snapshot::pubkey(self.id)
    }

    fn init(&mut self, id: u64) -> Result<()> {
        self.id = id;
        self.total_frames = 0;
        self.total_stake = 0;
        Ok(())
    }
}
