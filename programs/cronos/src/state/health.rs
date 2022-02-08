use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use std::convert::TryFrom;

pub const SEED_HEALTH: &[u8] = b"health";

#[account]
#[derive(Debug)]
pub struct Health {
    pub real_time: i64,
    pub target_time: i64,
    pub bump: u8,
}

impl TryFrom<Vec<u8>> for Health {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Health::try_deserialize(&mut data.as_slice())
    }
}

impl Health {
    pub fn find_pda() -> PDA {
        Pubkey::find_program_address(&[SEED_HEALTH], &crate::ID)
    }
}
