use crate::{errors::{RegistryError, AccountError}, state::NodeAccount};

use super::Node;

use {
    crate::pda::PDA,
    super::{RegistryPage, RegistryPageAccount},
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_REGISTRY: &[u8] = b"registry";

/**
 * Registry
 */

#[account]
#[derive(Debug)]
pub struct Registry {
    pub bump: u8,
    pub is_locked: bool,
    pub last_snapshot_ts: Option<i64>,
    pub node_count: u64,
    pub page_count: u64,
    pub token_mint: Pubkey,
}

impl Registry {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_REGISTRY],  &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Registry {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Registry::try_deserialize(&mut data.as_slice())
    }
}

/**
 * RegistryAccount
 */

pub trait RegistryAccount {
    fn new(&mut self, bump: u8, token_mint: Pubkey) -> Result<()>;

    fn new_node(
        &mut self, 
        authority: &mut Signer, 
        node: &mut Account<Node>, 
        node_bump: u8, 
        registry_page: &mut Account<RegistryPage>
    ) -> Result<()>;

    fn new_page(&mut self, page: &mut Account<RegistryPage>, page_bump: u8) -> Result<()>;

    fn lock(&mut self) -> Result<()>;

    fn unlock(&mut self, ts: i64) -> Result<()>;
}

impl RegistryAccount for Account<'_, Registry> {
    fn new(&mut self, bump: u8, token_mint: Pubkey) -> Result<()> {
        require!(self.bump == 0, AccountError::AlreadyInitialized);
        self.bump = bump;
        self.is_locked = false;
        self.last_snapshot_ts = None;
        self.node_count = 0;
        self.page_count = 0;
        self.token_mint = token_mint;
        Ok(())
    }

    fn new_node(
        &mut self, 
        authority: &mut Signer, 
        node: &mut Account<Node>, 
        node_bump: u8, 
        registry_page: &mut Account<RegistryPage>
    ) -> Result<()> {
        require!(!self.is_locked, RegistryError::Locked);
        node.new(authority, node_bump).unwrap();
        registry_page.append(node).unwrap();
        self.node_count = self.node_count.checked_add(1).unwrap();
        Ok(())
    }

    fn new_page(&mut self, page: &mut Account<RegistryPage>, page_bump: u8) -> Result<()> {
        require!(!self.is_locked, RegistryError::Locked);
        page.new(page_bump, self.page_count).unwrap();
        self.page_count = self.page_count.checked_add(1).unwrap();
        Ok(())
    }

    fn lock(&mut self) -> Result<()> {
        self.is_locked = true;
        Ok(())
    }

    fn unlock(&mut self, ts: i64) -> Result<()> {
        self.is_locked = false;
        self.last_snapshot_ts = Some(ts);
        Ok(())
    }
}
