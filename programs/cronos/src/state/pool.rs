use {
    crate::pda::PDA,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_POOL: &[u8] = b"pool";

/**
 * Pool
 */

#[account]
#[derive(Debug)]
pub struct Pool {
    pub workers: Vec<Pubkey>,
    pub bump: u8,
}

impl Pool {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_POOL], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Pool {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Pool::try_deserialize(&mut data.as_slice())
    }
}

/**
 * PoolAccount
 */

pub trait PoolAccount {
    fn init(&mut self, bump: u8) -> Result<()>;
}

impl PoolAccount for Account<'_, Pool> {
    fn init(&mut self, bump: u8) -> Result<()> {
        self.bump = bump;
        Ok(())
    }
}
