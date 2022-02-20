use crate::errors::ErrorCode;
use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use std::convert::TryFrom;

pub const SEED_CONFIG: &[u8] = b"config";

/**
 * Config
 */

#[account]
#[derive(Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub min_recurr: i64,
    pub program_fee: u64,
    pub worker_fee: u64,
    pub bump: u8,
}

impl Config {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_CONFIG], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Config {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Config::try_deserialize(&mut data.as_slice())
    }
}

/**
 * ConfigAccount
 */

pub trait ConfigAccount {
    fn init(&mut self, admin: Pubkey, bump: u8) -> ProgramResult;
    fn update_admin(&mut self, admin: &Signer, new_admin: Pubkey) -> ProgramResult;
    fn update_min_recurr(&mut self, admin: &Signer, new_min_recurr: i64) -> ProgramResult;
    fn update_program_fee(&mut self, admin: &Signer, new_program_fee: u64) -> ProgramResult;
    fn update_worker_fee(&mut self, admin: &Signer, new_worker_fee: u64) -> ProgramResult;
}

impl ConfigAccount for Account<'_, Config> {
    fn init(&mut self, admin: Pubkey, bump: u8) -> ProgramResult {
        self.admin = admin;
        self.min_recurr = 5; // Minimum supported recurrence interval
        self.program_fee = 0; // Lamports to pay to program for each task execution
        self.worker_fee = 0; // Lamports to pay to worker for each task execution
        self.bump = bump;
        Ok(())
    }

    fn update_admin(&mut self, admin: &Signer, new_admin: Pubkey) -> ProgramResult {
        require!(self.admin == admin.key(), ErrorCode::NotAuthorizedAdmin);
        self.admin = new_admin;
        Ok(())
    }

    fn update_min_recurr(&mut self, admin: &Signer, new_min_recurr: i64) -> ProgramResult {
        require!(self.admin == admin.key(), ErrorCode::NotAuthorizedAdmin);
        require!(new_min_recurr >= 0, ErrorCode::InvalidRecurrNegative);
        self.min_recurr = new_min_recurr;
        Ok(())
    }

    fn update_program_fee(&mut self, admin: &Signer, new_program_fee: u64) -> ProgramResult {
        require!(self.admin == admin.key(), ErrorCode::NotAuthorizedAdmin);
        self.program_fee = new_program_fee;
        Ok(())
    }

    fn update_worker_fee(&mut self, admin: &Signer, new_worker_fee: u64) -> ProgramResult {
        require!(self.admin == admin.key(), ErrorCode::NotAuthorizedAdmin);
        self.worker_fee = new_worker_fee;
        Ok(())
    }
}
