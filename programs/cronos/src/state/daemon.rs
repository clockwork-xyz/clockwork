use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use solana_program::instruction::Instruction;
use solana_program::program::invoke_signed;
use std::convert::TryFrom;

pub const SEED_DAEMON: &[u8] = b"daemon";

#[account]
#[derive(Debug)]
pub struct Daemon {
    pub owner: Pubkey,
    pub task_count: u128,
    pub bump: u8,
}

impl TryFrom<Vec<u8>> for Daemon {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Daemon::try_deserialize(&mut data.as_slice())
    }
}

impl Daemon {
    pub fn pda(owner: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_DAEMON, owner.as_ref()], &crate::ID)
    }
}

impl Daemon {
    pub fn initialize(&mut self, owner: Pubkey, bump: u8) -> ProgramResult {
        self.owner = owner;
        self.task_count = 0;
        self.bump = bump;
        Ok(())
    }

    pub fn invoke(&self, ix: &Instruction, account_infos: &[AccountInfo]) -> ProgramResult {
        invoke_signed(
            ix,
            account_infos,
            &[&[SEED_DAEMON, self.owner.key().as_ref(), &[self.bump]]],
        )
    }
}
