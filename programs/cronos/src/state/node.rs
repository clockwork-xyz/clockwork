use {
    crate::pda::PDA,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_NODE: &[u8] = b"node";

/**
 * Node
 */

#[account]
#[derive(Debug)]
pub struct Node {
    pub owner: Pubkey,
    pub stake: u64,
    pub bump: u8,
}

impl Node {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_NODE], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Node {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Node::try_deserialize(&mut data.as_slice())
    }
}

/**
 * NodeAccount
 */

pub trait NodeAccount {
    fn open(&mut self, bump: u8, owner: Pubkey) -> Result<()>;
}

impl NodeAccount for Account<'_, Node> {
    fn open(&mut self, bump: u8, owner: Pubkey) -> Result<()> {
        self.owner = owner;
        self.stake = 0;
        self.bump = bump;
        Ok(())
    }
}
