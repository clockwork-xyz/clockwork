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
    pub delegate: Pubkey, // The node's delegate address used to sign queue_exec ixs
    pub id: u64,
    pub owner: Pubkey, // The node's owner gossip network (controls the stake)
    pub stake: Pubkey, // The associated token account
}

impl Node {
    pub fn pda(delegate: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_NODE, delegate.as_ref()], &crate::ID)
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
        delegate: &Signer,
        id: u64,
        owner: &mut Signer,
        stake: &mut Account<TokenAccount>,
    ) -> Result<()>;
}

impl NodeAccount for Account<'_, Node> {
    fn new(
        &mut self,
        delegate: &Signer,
        id: u64,
        owner: &mut Signer,
        stake: &mut Account<TokenAccount>,
    ) -> Result<()> {
        self.delegate = delegate.key();
        self.id = id;
        self.owner = owner.key();
        self.stake = stake.key();
        Ok(())
    }
}
