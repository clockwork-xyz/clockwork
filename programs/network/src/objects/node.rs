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
    pub stake: Pubkey,                    // The associated token account
    pub worker: Pubkey,                   // The node's worker address (used to sign txs)
    pub supported_pools: HashSet<Pubkey>, // The set pools that this node supports
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
 * NodeSettings
 */
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct NodeSettings {
    pub supported_pools: HashSet<Pubkey>,
}

/**
 * NodeAccount
 */

pub trait NodeAccount {
    fn pubkey(&self) -> Pubkey;

    fn new(
        &mut self,
        authority: &mut Signer,
        id: u64,
        stake: &mut Account<TokenAccount>,
        worker: &Signer,
    ) -> Result<()>;

    fn update(&mut self, settings: NodeSettings) -> Result<()>;
}

impl NodeAccount for Account<'_, Node> {
    fn pubkey(&self) -> Pubkey {
        Node::pubkey(self.id)
    }

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

    fn update(&mut self, settings: NodeSettings) -> Result<()> {
        self.supported_pools = settings.supported_pools;
        Ok(())
    }
}
