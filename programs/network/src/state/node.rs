use {
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
    pub owner: Pubkey,  // The node's owner gossip network (controls the stake)
    pub stake: Pubkey,  // The associated token account
    pub worker: Pubkey, // The node's worker address used to sign txs
}

impl Node {
    pub fn pubkey(worker: Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[SEED_NODE, worker.as_ref()], &crate::ID).0
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
        owner: &mut Signer,
        stake: &mut Account<TokenAccount>,
        worker: &Signer,
    ) -> Result<()>;
}

impl NodeAccount for Account<'_, Node> {
    fn new(
        &mut self,
        id: u64,
        owner: &mut Signer,
        stake: &mut Account<TokenAccount>,
        worker: &Signer,
    ) -> Result<()> {
        self.id = id;
        self.owner = owner.key();
        self.stake = stake.key();
        self.worker = worker.key();
        Ok(())
    }
}
