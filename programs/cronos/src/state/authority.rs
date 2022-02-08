use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use std::convert::TryFrom;

pub const SEED_AUTHORITY: &[u8] = b"authority";

#[account]
#[derive(Debug)]
pub struct Authority {
    pub bump: u8,
}

impl TryFrom<Vec<u8>> for Authority {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Authority::try_deserialize(&mut data.as_slice())
    }
}

impl Authority {
    pub fn find_pda() -> PDA {
        Pubkey::find_program_address(&[SEED_AUTHORITY], &crate::ID)
    }
}
