use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;

pub const SEED_TREASURY: &[u8] = b"treasury";

#[account]
#[derive(Debug)]
pub struct Treasury {
    pub bump: u8,
}

impl From<Vec<u8>> for Treasury {
    fn from(data: Vec<u8>) -> Self {
        Treasury::try_deserialize(&mut data.as_slice()).unwrap()
    }
}
