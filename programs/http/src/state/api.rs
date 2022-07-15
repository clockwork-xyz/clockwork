use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_API: &[u8] = b"api";

/**
 * Api
 */

#[account]
#[derive(Debug)]
pub struct Api {
    pub ack_authority: Pubkey,
    pub base_url: String,
    pub owner: Pubkey,
    pub request_count: u128,
}

impl Api {
    pub fn pubkey(base_url: String, owner: Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_API, base_url.as_bytes().as_ref(), owner.as_ref()],
            &crate::ID,
        )
        .0
    }
}

impl TryFrom<Vec<u8>> for Api {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Api::try_deserialize(&mut data.as_slice())
    }
}

/**
 * ApiAccount
 */

pub trait ApiAccount {
    fn new(&mut self, ack_authority: Pubkey, base_url: String, owner: Pubkey) -> Result<()>;
}

impl ApiAccount for Account<'_, Api> {
    fn new(&mut self, ack_authority: Pubkey, base_url: String, owner: Pubkey) -> Result<()> {
        self.ack_authority = ack_authority;
        self.base_url = base_url;
        self.owner = owner;
        self.request_count = 0;
        Ok(())
    }
}
