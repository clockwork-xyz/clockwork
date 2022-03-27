use super::RegistryPageAccount;

use {
    crate::pda::PDA,
    super::RegistryPage,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_REGISTRY: &[u8] = b"registry";

/**
 * Registry
 */

#[account]
#[derive(Debug)]
pub struct Registry {
    pub bump: u8,
    pub is_locked: bool,
    pub node_count: u64,
    pub page_count: u64,
    pub token_count: u64,
    pub token_mint: Pubkey,
    pub token_stake: u64,
}

impl Registry {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_REGISTRY],  &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Registry {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Registry::try_deserialize(&mut data.as_slice())
    }
}

/**
 * RegistryAccount
 */

pub trait RegistryAccount {
    fn new(&mut self, bump: u8, token_mint: Pubkey) -> Result<()>;

    fn lock(&mut self) -> Result<()>;
    fn unlock(&mut self) -> Result<()>;

    fn new_page(&mut self, page: &mut Account<RegistryPage>, page_bump: u8) -> Result<()>;
}

impl RegistryAccount for Account<'_, Registry> {
    fn new(&mut self, bump: u8, token_mint: Pubkey) -> Result<()> {
        self.bump = bump;
        self.is_locked = false;
        self.node_count = 0;
        self.page_count = 0;
        self.token_count = 0;
        self.token_mint = token_mint;
        self.token_stake = 0;
        Ok(())
    }

    fn new_page(&mut self, page: &mut Account<RegistryPage>, page_bump: u8) -> Result<()> {
        page.new(page_bump, self.page_count).unwrap();
        self.page_count = self.page_count.checked_add(1).unwrap();
        Ok(())
    }

    fn lock(&mut self) -> Result<()> {
        self.is_locked = true;
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        self.is_locked = false;
        Ok(())
    }
}
