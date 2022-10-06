use {
    super::{SnapshotEntry, Worker},
    crate::objects::SnapshotEntryAccount,
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
    pub status: SnapshotStatus,
    pub total_stake: u64,
    pub total_workers: u64,
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

    fn capture(
        &mut self,
        entry: &mut Account<SnapshotEntry>,
        node: &Account<Worker>,
        stake: &Account<TokenAccount>,
    ) -> Result<()>;
}

impl SnapshotAccount for Account<'_, Snapshot> {
    fn pubkey(&self) -> Pubkey {
        Snapshot::pubkey(self.epoch)
    }

    fn init(&mut self, epoch: Pubkey) -> Result<()> {
        self.epoch = epoch;
        self.status = SnapshotStatus::Capturing;
        self.total_stake = 0;
        self.total_workers = 0;
        Ok(())
    }

    fn capture(
        &mut self,
        entry: &mut Account<SnapshotEntry>,
        worker: &Account<Worker>,
        stake: &Account<TokenAccount>,
    ) -> Result<()> {
        // Record the new snapshot entry
        entry.init(
            self.total_workers,
            self.key(),
            self.total_stake,
            stake.amount,
            worker.delegate,
        )?;

        // Update the snapshot's entry count
        self.total_workers = self.total_workers.checked_add(1).unwrap();

        // Update the sum stake amount
        self.total_stake = self.total_stake.checked_add(stake.amount).unwrap();

        Ok(())
    }
}

/**
 * SnapshotStatus
 */
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub enum SnapshotStatus {
    Archived,
    Capturing,
    Closing,
    Current,
}
