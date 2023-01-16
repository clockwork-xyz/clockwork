use anchor_lang::{prelude::*, AnchorDeserialize};
use clockwork_macros::TryFromData;

pub const SEED_UNSTAKE: &[u8] = b"unstake";

/// Unstake
#[account]
#[derive(Debug, TryFromData)]
pub struct Unstake {
    pub amount: u64,
    pub authority: Pubkey,
    pub delegation: Pubkey,
    pub id: u64,
    pub worker: Pubkey,
}

impl Unstake {
    pub fn pubkey(id: u64) -> Pubkey {
        Pubkey::find_program_address(&[SEED_UNSTAKE, id.to_be_bytes().as_ref()], &crate::ID).0
    }
}

/// UnstakeAccount
pub trait UnstakeAccount {
    fn pubkey(&self) -> Pubkey;

    fn init(
        &mut self,
        amount: u64,
        authority: Pubkey,
        delegation: Pubkey,
        id: u64,
        worker: Pubkey,
    ) -> Result<()>;
}

impl UnstakeAccount for Account<'_, Unstake> {
    fn pubkey(&self) -> Pubkey {
        Unstake::pubkey(self.id)
    }

    fn init(
        &mut self,
        amount: u64,
        authority: Pubkey,
        delegation: Pubkey,
        id: u64,
        worker: Pubkey,
    ) -> Result<()> {
        self.amount = amount;
        self.authority = authority.key();
        self.delegation = delegation;
        self.id = id;
        self.worker = worker;
        Ok(())
    }
}
