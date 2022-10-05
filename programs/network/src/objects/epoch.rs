use anchor_lang::{prelude::*, AnchorDeserialize};

use super::{Snapshot, SnapshotAccount};

pub const SEED_EPOCH: &[u8] = b"epoch";

/**
 * Epoch
 */

#[account]
#[derive(Debug)]
pub struct Epoch {
    pub id: u64,
    pub snapshot: Snapshot,
    pub status: Pubkey,
}

impl Epoch {
    pub fn pubkey(id: u64) -> Pubkey {
        Pubkey::find_program_address(&[SEED_EPOCH, id.to_be_bytes().as_ref()], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Epoch {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Epoch::try_deserialize(&mut data.as_slice())
    }
}

/**
 * EpochAccount
 */

pub trait EpochAccount {
    fn pubkey(&self) -> Pubkey;
}

impl EpochAccount for Account<'_, Epoch> {
    fn pubkey(&self) -> Pubkey {
        Epoch::pubkey(self.id)
    }
}

///
/// EpochStatus
///
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum EpochStatus {
    Current,
}
