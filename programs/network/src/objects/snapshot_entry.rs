use anchor_lang::{prelude::*, AnchorDeserialize};

pub const SEED_SNAPSHOT_ENTRY: &[u8] = b"snapshot_entry";

/**
 * SnapshotEntry
 */

#[account]
#[derive(Debug)]
pub struct SnapshotEntry {
    pub delegation: Pubkey,
    pub frame: Pubkey,
    pub id: u64,
    pub stake_balance: u64,
}

impl SnapshotEntry {
    pub fn pubkey(frame: Pubkey, id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[
                SEED_SNAPSHOT_ENTRY,
                frame.as_ref(),
                id.to_be_bytes().as_ref(),
            ],
            &crate::ID,
        )
        .0
    }
}

impl TryFrom<Vec<u8>> for SnapshotEntry {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        SnapshotEntry::try_deserialize(&mut data.as_slice())
    }
}

/**
 * SnapshotEntryAccount
 */

pub trait SnapshotEntryAccount {
    fn pubkey(&self) -> Pubkey;

    fn init(
        &mut self,
        delegation: Pubkey,
        frame: Pubkey,
        id: u64,
        stake_balance: u64,
    ) -> Result<()>;
}

impl SnapshotEntryAccount for Account<'_, SnapshotEntry> {
    fn pubkey(&self) -> Pubkey {
        SnapshotEntry::pubkey(self.frame, self.id)
    }

    fn init(
        &mut self,
        delegation: Pubkey,
        frame: Pubkey,
        id: u64,
        stake_balance: u64,
    ) -> Result<()> {
        self.delegation = delegation;
        self.frame = frame;
        self.id = id;
        self.stake_balance = stake_balance;
        Ok(())
    }
}
