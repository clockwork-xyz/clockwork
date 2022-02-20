use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use std::convert::TryFrom;

use super::Config;

pub const SEED_HEALTH: &[u8] = b"health";

/**
 * Health
 */

#[account]
#[derive(Debug)]
pub struct Health {
    pub last_ping: i64,
    pub target_ping: i64,
    pub bump: u8,
}

impl Health {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_HEALTH], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Health {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Health::try_deserialize(&mut data.as_slice())
    }
}

/**
 * HealthAccount
 */

pub trait HealthAccount {
    fn init(&mut self, bump: u8) -> ProgramResult;
    fn ping(&mut self, clock: &Sysvar<Clock>, config: &Account<Config>) -> ProgramResult;
    fn reset(&mut self, clock: &Sysvar<Clock>) -> ProgramResult;
}

impl HealthAccount for Account<'_, Health> {
    fn init(&mut self, bump: u8) -> ProgramResult {
        self.last_ping = 0;
        self.target_ping = 0;
        self.bump = bump;
        Ok(())
    }

    fn ping(&mut self, clock: &Sysvar<Clock>, config: &Account<Config>) -> ProgramResult {
        self.last_ping = clock.unix_timestamp;
        self.target_ping = self.target_ping.checked_add(config.min_recurr).unwrap();
        Ok(())
    }

    fn reset(&mut self, clock: &Sysvar<Clock>) -> ProgramResult {
        self.last_ping = clock.unix_timestamp;
        self.target_ping = clock.unix_timestamp;
        Ok(())
    }
}
