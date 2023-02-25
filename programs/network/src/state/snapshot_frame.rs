use anchor_lang::{prelude::*, AnchorDeserialize};

pub const SEED_SNAPSHOT_FRAME: &[u8] = b"snapshot_frame";

/**
 * SnapshotFrame
 */
#[account]
#[derive(Debug)]
pub struct SnapshotFrame {
    pub id: u64,
    pub snapshot: Pubkey,
    pub stake_amount: u64,
    pub stake_offset: u64,
    pub total_entries: u64,
    pub worker: Pubkey,
}

impl SnapshotFrame {
    pub fn pubkey(snapshot: Pubkey, id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[
                SEED_SNAPSHOT_FRAME,
                snapshot.as_ref(),
                id.to_be_bytes().as_ref(),
            ],
            &crate::ID,
        )
        .0
    }
}

/**
 * SnapshotFrameAccount
 */

pub trait SnapshotFrameAccount {
    fn pubkey(&self) -> Pubkey;

    fn init(
        &mut self,
        id: u64,
        snapshot: Pubkey,
        stake_amount: u64,
        stake_offset: u64,
        worker: Pubkey,
    ) -> Result<()>;
}

impl SnapshotFrameAccount for Account<'_, SnapshotFrame> {
    fn pubkey(&self) -> Pubkey {
        SnapshotFrame::pubkey(self.snapshot, self.id)
    }

    fn init(
        &mut self,
        id: u64,
        snapshot: Pubkey,
        stake_amount: u64,
        stake_offset: u64,
        worker: Pubkey,
    ) -> Result<()> {
        self.id = id;
        self.snapshot = snapshot;
        self.stake_offset = stake_offset;
        self.stake_amount = stake_amount;
        self.total_entries = 0;
        self.worker = worker;
        Ok(())
    }
}
