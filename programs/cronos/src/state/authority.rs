use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use std::convert::TryFrom;

pub const SEED_AUTHORITY: &[u8] = b"authority";

/**
 * Authority
 */

#[account]
#[derive(Debug)]
pub struct Authority {
    pub bump: u8,
}

impl Authority {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_AUTHORITY], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Authority {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Authority::try_deserialize(&mut data.as_slice())
    }
}

/**
 * AuthorityAccount
 */

pub trait AuthorityAccount {
    fn init(&mut self, bump: u8) -> ProgramResult;
}

impl AuthorityAccount for Account<'_, Authority> {
    fn init(&mut self, bump: u8) -> ProgramResult {
        self.bump = bump;
        Ok(())
    }
}
