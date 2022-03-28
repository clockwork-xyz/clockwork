use {
    crate::{errors::CronosError, pda::PDA},
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
    pub authority: Pubkey,
    pub bump: u8,
    pub stake: u64,
}

impl Node {
    pub fn pda(authority: Pubkey) -> PDA {
        Pubkey::find_program_address(&[
            SEED_NODE,
            authority.as_ref(),
        ], &crate::ID)
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
    fn new(&mut self, authority: &mut Signer, bump: u8) -> Result<()>;

    fn stake(&mut self, _amount: u64, _authority: &mut Signer) -> Result<()>;

    fn unstake(&mut self, _amount: u64, _authority: &mut Signer) -> Result<()>;
}

impl NodeAccount for Account<'_, Node> {
    fn new(&mut self, authority: &mut Signer, bump: u8) -> Result<()> {
        require!(self.bump == 0, CronosError::AccountAlreadyInitialized);
        self.authority = authority.key();
        self.bump = bump;
        self.stake = 0;
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
