use crate::errors::ErrorCode;
use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use solana_program::instruction::Instruction;
use solana_program::program::invoke_signed;
use std::convert::TryFrom;

pub const SEED_DAEMON: &[u8] = b"daemon";

/**
 * Daemon
 */

#[account]
#[derive(Debug)]
pub struct Daemon {
    pub owner: Pubkey,
    pub task_count: u128,
    pub bump: u8,
}

impl Daemon {
    pub fn pda(owner: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_DAEMON, owner.as_ref()], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Daemon {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Daemon::try_deserialize(&mut data.as_slice())
    }
}

/**
 * DaemonAccount
 */

pub trait DaemonAccount {
    fn init(&mut self, owner: Pubkey, bump: u8) -> ProgramResult;
    fn invoke(&mut self, ix: &Instruction, account_infos: &[AccountInfo]) -> ProgramResult;
    fn widthdraw(&mut self, amount: u64, owner: &Signer) -> ProgramResult;
}

impl DaemonAccount for Account<'_, Daemon> {
    fn init(&mut self, owner: Pubkey, bump: u8) -> ProgramResult {
        self.owner = owner;
        self.task_count = 0;
        self.bump = bump;
        Ok(())
    }

    fn invoke(&mut self, ix: &Instruction, account_infos: &[AccountInfo]) -> ProgramResult {
        invoke_signed(
            ix,
            account_infos,
            &[&[SEED_DAEMON, self.owner.key().as_ref(), &[self.bump]]],
        )
    }

    fn widthdraw(&mut self, amount: u64, owner: &Signer) -> ProgramResult {
        require!(
            owner.key() == self.owner,
            ErrorCode::NotAuthorizedDaemonOwner
        );

        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(amount)
            .unwrap();
        **owner.to_account_info().try_borrow_mut_lamports()? = owner
            .to_account_info()
            .lamports()
            .checked_add(amount)
            .unwrap();

        Ok(())
    }
}
