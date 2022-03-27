use {
    crate::pda::PDA,
    anchor_lang::{AnchorDeserialize, prelude::*},
    std::convert::TryFrom,
};

pub const SEED_REGISTRY_PAGE: &[u8] = b"registry_page";

/**
 * RegistryPage
 */

#[account]
#[derive(Debug)]
pub struct RegistryPage {
    pub bump: u8,
    pub id: u64,
    pub nodes: Vec<Pubkey>,
}

impl RegistryPage {
    pub fn pda(id: u64) -> PDA {
        Pubkey::find_program_address(
            &[
                SEED_REGISTRY_PAGE,
                id.to_be_bytes().as_ref()
            ], 
            &crate::ID
        )
    }
}

impl TryFrom<Vec<u8>> for RegistryPage {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        RegistryPage::try_deserialize(&mut data.as_slice())
    }
}

/**
 * RegistryPageAccount
 */

pub trait RegistryPageAccount {
    fn new(&mut self, bump: u8, id: u64) -> Result<()>;
}

impl RegistryPageAccount for Account<'_, RegistryPage> {
    fn new(&mut self, bump: u8, id: u64) -> Result<()> {
        self.bump = bump;
        self.id = id;
        self.nodes = vec![];
        Ok(())
    }
}
