use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    anchor_spl::token::TokenAccount,
    std::{collections::HashSet, convert::TryFrom},
};

pub const SEED_NODE: &[u8] = b"node";

/**
 * Node
 */

#[account]
#[derive(Debug)]
pub struct Node {
    pub authority: Pubkey, // The node's authority (controls the stake)
    pub id: u64,
    pub stake: Pubkey,  // The associated token account
    pub worker: Pubkey, // The node's worker address (used to sign txs)
    pub supported_pools: HashSet<Pubkey>,
}

impl Node {
    pub fn pubkey(id: u64) -> Pubkey {
        Pubkey::find_program_address(&[SEED_NODE, id.to_be_bytes().as_ref()], &crate::ID).0
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
        authority: &mut Signer,
        id: u64,
        stake: &mut Account<TokenAccount>,
        worker: &Signer,
    ) -> Result<()>;
}

impl NodeAccount for Account<'_, Node> {
    fn new(
        &mut self,
        authority: &mut Signer,
        id: u64,
        stake: &mut Account<TokenAccount>,
        worker: &Signer,
    ) -> Result<()> {
        self.authority = authority.key();
        self.id = id;
        self.stake = stake.key();
        self.worker = worker.key();
        Ok(())
    }
}
