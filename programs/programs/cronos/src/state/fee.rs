use super::Treasury;
use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;

use std::convert::TryFrom;

pub const SEED_FEE: &[u8] = b"fee";

/**
 * Fee
 */

#[account]
#[derive(Debug)]
pub struct Fee {
    pub daemon: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

impl TryFrom<Vec<u8>> for Fee {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Fee::try_deserialize(&mut data.as_slice())
    }
}

impl Fee {
    pub fn pda(daemon: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_FEE, daemon.as_ref()], &crate::ID)
    }
}

/**
 * FeeAccount
 */

pub trait FeeAccount {
    fn init(&mut self, daemon: Pubkey, bump: u8) -> Result<()>;

    fn collect(&mut self, treasury: &mut Account<Treasury>) -> Result<()>;
}

impl FeeAccount for Account<'_, Fee> {
    fn init(&mut self, daemon: Pubkey, bump: u8) -> Result<()> {
        self.daemon = daemon;
        self.balance = 0;
        self.bump = bump;
        Ok(())
    }

    fn collect(&mut self, treasury: &mut Account<Treasury>) -> Result<()> {
        // Collect lamports from fee account to treasury.
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(self.balance)
            .unwrap();
        **treasury.to_account_info().try_borrow_mut_lamports()? = treasury
            .to_account_info()
            .lamports()
            .checked_add(self.balance)
            .unwrap();

        // Zero out the collectable balance.
        self.balance = 0;

        Ok(())
    }
}
