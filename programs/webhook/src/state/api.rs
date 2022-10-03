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
    pub authority: Pubkey,
    pub base_url: String,
    pub request_count: u64,
}

impl Api {
    pub fn pubkey(authority: Pubkey, base_url: String) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_API, authority.as_ref(), base_url.as_bytes().as_ref()],
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
    fn init(&mut self, ack_authority: Pubkey, authority: Pubkey, base_url: String) -> Result<()>;
}

impl ApiAccount for Account<'_, Api> {
    fn init(&mut self, ack_authority: Pubkey, authority: Pubkey, base_url: String) -> Result<()> {
        self.ack_authority = ack_authority;
        self.authority = authority;
        self.base_url = base_url;
        self.request_count = 0;
        Ok(())
    }
}
