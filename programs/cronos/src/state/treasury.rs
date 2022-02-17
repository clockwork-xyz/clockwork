use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;
use std::convert::TryFrom;

pub const SEED_TREASURY: &[u8] = b"treasury";

#[account]
#[derive(Debug)]
pub struct Treasury {
    pub bump: u8,
}

impl TryFrom<Vec<u8>> for Treasury {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Treasury::try_deserialize(&mut data.as_slice())
    }
}

impl Treasury {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_TREASURY], &crate::ID)
    }
}
