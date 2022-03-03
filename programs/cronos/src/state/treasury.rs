use {
    crate::pda::PDA,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_TREASURY: &[u8] = b"treasury";

/**
 * Treasury
 */

#[account]
#[derive(Debug)]
pub struct Treasury {
    pub bump: u8,
}

impl Treasury {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_TREASURY], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Treasury {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Treasury::try_deserialize(&mut data.as_slice())
    }
}

/**
 * TreasuryAccount
 */

pub trait TreasuryAccount {
    fn init(&mut self, bump: u8) -> Result<()>;
}

impl TreasuryAccount for Account<'_, Treasury> {
    fn init(&mut self, bump: u8) -> Result<()> {
        self.bump = bump;
        Ok(())
    }
}
