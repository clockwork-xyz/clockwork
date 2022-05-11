use {
    crate::pda::PDA,
    anchor_lang::{prelude::*, AnchorDeserialize},
    anchor_spl::token::TokenAccount,
    std::convert::TryFrom,
};

pub const SEED_NODE: &[u8] = b"node";

/**
 * Node
 */

#[account]
#[derive(Debug)]
pub struct Node {
    pub id: u64,
    pub identity: Pubkey, // The node's address on the Solana gossip network
    pub stake: Pubkey,    // The associated token account
}

impl Node {
    pub fn pda(identity: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_NODE, identity.as_ref()], &crate::ID)
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
    fn new(
        &mut self,
        id: u64,
        identity: &mut Signer,
        stake: &mut Account<TokenAccount>,
    ) -> Result<()>;
}

impl NodeAccount for Account<'_, Node> {
    fn new(
        &mut self,
        id: u64,
        identity: &mut Signer,
        stake: &mut Account<TokenAccount>,
    ) -> Result<()> {
        self.id = id;
        self.identity = identity.key();
        self.stake = stake.key();
        Ok(())
    }
}
