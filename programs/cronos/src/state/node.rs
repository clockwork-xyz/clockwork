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
    pub bump: u8,
    pub int: u128,
    pub owner: Pubkey,
    pub stake: u64,
}

impl Node {
    pub fn pda(owner: Pubkey, int: u128) -> PDA {
        Pubkey::find_program_address(
            &[SEED_NODE, owner.as_ref(), int.to_be_bytes().as_ref()], 
            &crate::ID
        )
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
    fn open(&mut self, bump: u8, int: u128, owner: Pubkey) -> Result<()>;
    fn stake(&mut self, amount: u64) -> Result<()>;
}

impl NodeAccount for Account<'_, Node> {
    fn open(&mut self, bump: u8, int: u128, owner: Pubkey) -> Result<()> {
        self.bump = bump;
        self.int = int;
        self.owner = owner;
        self.stake = 0;
        Ok(())
    }

    fn stake(&mut self, amount: u64) -> Result<()> {
        self.stake = self.stake.checked_add(amount).unwrap();
        Ok(())
    }
}
