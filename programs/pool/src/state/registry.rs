use {
    crate::{errors::CronosError, state::{NodeAccount, SnapshotAccount}, pda::PDA},
    super::{Node, Snapshot, RegistryPage, RegistryPageAccount},
    anchor_lang::{prelude::*, AnchorDeserialize},
    anchor_spl::token::Mint,
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
    pub mint: Pubkey,
    pub node_count: u64,
    pub page_count: u64,
    pub snapshot_count: u64
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
    fn new(&mut self, bump: u8, mint: &Account<Mint>) -> Result<()>;

    fn new_node(
        &mut self, 
        authority: &mut Signer, 
        node: &mut Account<Node>, 
        node_bump: u8, 
        registry_page: &mut Account<RegistryPage>
    ) -> Result<()>;

    fn new_page(
        &mut self, 
        page: &mut Account<RegistryPage>, 
        page_bump: u8
    ) -> Result<()>;

    fn new_snapshot(
        &mut self,
        snapshot: &mut Account<Snapshot>,
        snapshot_bump: u8
    ) -> Result<()>;

    fn lock(&mut self) -> Result<()>;

    fn unlock(&mut self) -> Result<()>;
}

impl RegistryAccount for Account<'_, Registry> {
    fn new(&mut self, bump: u8, mint: &Account<Mint>) -> Result<()> {
        require!(self.bump == 0, CronosError::AccountAlreadyInitialized);
        self.bump = bump;
        self.is_locked = false;
        self.node_count = 0;
        self.page_count = 0;
        self.snapshot_count = 0;
        self.mint = mint.key();
        Ok(())
    }

    fn new_node(
        &mut self, 
        authority: &mut Signer, 
        node: &mut Account<Node>, 
        node_bump: u8, 
        registry_page: &mut Account<RegistryPage>
    ) -> Result<()> {
        require!(!self.is_locked, CronosError::RegistryLocked);
        node.new(authority, node_bump)?;
        registry_page.append(node)?;
        self.node_count = self.node_count.checked_add(1).unwrap();
        Ok(())
    }

    fn new_page(&mut self, page: &mut Account<RegistryPage>, page_bump: u8) -> Result<()> {
        require!(!self.is_locked, CronosError::RegistryLocked);
        page.new(page_bump, self.page_count)?;
        self.page_count = self.page_count.checked_add(1).unwrap();
        Ok(())
    }

    fn new_snapshot(&mut self, snapshot: &mut Account<Snapshot>, snapshot_bump: u8) -> Result<()> {
        require!(!self.is_locked, CronosError::RegistryLocked);
        snapshot.new(snapshot_bump, self.snapshot_count)?;
        Ok(())
    }

    fn lock(&mut self) -> Result<()> {
        self.is_locked = true;
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        self.is_locked = false;
        Ok(())
    }

}
