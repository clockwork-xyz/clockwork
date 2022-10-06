use {
    super::{SnapshotFrame, SnapshotFrameAccount, Worker},
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
    pub epoch: Pubkey,
    pub total_frames: u64,
    pub total_stake: u64,
}

impl Snapshot {
    pub fn pubkey(epoch: Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[SEED_SNAPSHOT, epoch.as_ref()], &crate::ID).0
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
    fn pubkey(&self) -> Pubkey;

    fn init(&mut self, epoch: Pubkey) -> Result<()>;
}

impl SnapshotAccount for Account<'_, Snapshot> {
    fn pubkey(&self) -> Pubkey {
        Snapshot::pubkey(self.epoch)
    }

    fn init(&mut self, epoch: Pubkey) -> Result<()> {
        self.epoch = epoch;
        self.total_frames = 0;
        self.total_stake = 0;
        Ok(())
    }
}
