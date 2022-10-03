use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_AUTHORITY: &[u8] = b"authority";

/**
 * Authority
 */

#[account]
#[derive(Debug)]
pub struct Authority {}

impl Authority {
    pub fn pubkey() -> Pubkey {
        Pubkey::find_program_address(&[SEED_AUTHORITY], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Authority {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Authority::try_deserialize(&mut data.as_slice())
    }
}
