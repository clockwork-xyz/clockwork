use {
    crate::{errors::CronosError, pda::PDA},
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
    pub bump: u8,
    pub id: u64,
    pub identity: Pubkey,
    pub stake: Pubkey, // The associated token account
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
        bump: u8,
        id: u64,
        identity: &mut Signer,
        stake: &mut Account<TokenAccount>,
    ) -> Result<()>;

    fn stake(&mut self, _amount: u64, _authority: &mut Signer) -> Result<()>;

    fn unstake(&mut self, _amount: u64, _authority: &mut Signer) -> Result<()>;
}

impl NodeAccount for Account<'_, Node> {
    fn new(
        &mut self,
        bump: u8,
        id: u64,
        identity: &mut Signer,
        stake: &mut Account<TokenAccount>,
    ) -> Result<()> {
        require!(self.bump == 0, CronosError::AccountAlreadyInitialized);
        self.bump = bump;
        self.id = id;
        self.identity = identity.key();
        self.stake = stake.key();
        Ok(())
    }

    fn stake(&mut self, _amount: u64, _authority: &mut Signer) -> Result<()> {
        // TODO transfer tokens from authority token account to node token account

        Ok(())
    }

    fn unstake(&mut self, _amount: u64, _authority: &mut Signer) -> Result<()> {
        // TODO transfer tokens from node token account to authority token account

        Ok(())
    }
}
