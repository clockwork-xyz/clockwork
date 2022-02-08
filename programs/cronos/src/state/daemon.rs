use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
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
    pub fn find_pda(owner: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_DAEMON, owner.as_ref()], &crate::ID)
    }
}
