use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_MANAGER: &[u8] = b"manager";

/**
 * Manager
 */

#[account]
#[derive(Debug)]
pub struct Manager {
    pub authority: Pubkey,
    pub request_count: u128,
}

impl Manager {
    pub fn pubkey(authority: Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[SEED_MANAGER, authority.as_ref()], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Manager {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Manager::try_deserialize(&mut data.as_slice())
    }
}

/**
 * ManagerAccount
 */

pub trait ManagerAccount {
    fn new(&mut self, authority: Pubkey) -> Result<()>;
}

impl ManagerAccount for Account<'_, Manager> {
    fn new(&mut self, authority: Pubkey) -> Result<()> {
        self.authority = authority;
        self.request_count = 0;
        Ok(())
    }
}
