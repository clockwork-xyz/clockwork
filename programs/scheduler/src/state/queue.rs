use {
    crate::{errors::CronosError, pda::PDA},
    anchor_lang::{
        prelude::*,
        solana_program::{instruction::Instruction, program::invoke_signed},
        AnchorDeserialize,
    },
    std::convert::TryFrom,
};

pub const SEED_QUEUE: &[u8] = b"queue";

/**
 * Queue
 */

#[account]
#[derive(Debug)]
pub struct Queue {
    pub owner: Pubkey,
    pub task_count: u128,
    pub bump: u8,
}

impl Queue {
    pub fn pda(owner: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_QUEUE, owner.as_ref()], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Queue {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Queue::try_deserialize(&mut data.as_slice())
    }
}

/**
 * QueueAccount
 */

pub trait QueueAccount {
    fn new(&mut self, bump: u8, owner: Pubkey) -> Result<()>;

    fn sign(&self, ix: &Instruction, account_infos: &[AccountInfo]) -> Result<()>;
}

impl QueueAccount for Account<'_, Queue> {
    fn new(&mut self, bump: u8, owner: Pubkey) -> Result<()> {
        self.bump = bump;
        self.owner = owner;
        self.task_count = 0;
        Ok(())
    }

    fn sign(&self, ix: &Instruction, account_infos: &[AccountInfo]) -> Result<()> {
        invoke_signed(
            ix,
            account_infos,
            &[&[SEED_QUEUE, self.owner.as_ref(), &[self.bump]]],
        )
        .map_err(|_err| CronosError::InnerIxFailed.into())
    }
}
