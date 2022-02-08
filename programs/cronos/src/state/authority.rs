use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;

pub const SEED_AUTHORITY: &[u8] = b"authority";

#[account]
#[derive(Debug)]
pub struct Authority {
    pub bump: u8,
}

impl From<Vec<u8>> for Authority {
    fn from(data: Vec<u8>) -> Self {
        Authority::try_deserialize(&mut data.as_slice()).unwrap()
    }
}
