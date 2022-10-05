use {
    super::{Node, Snapshot},
    crate::{
        errors::ClockworkError,
        objects::{NodeAccount, SnapshotAccount, SnapshotStatus},
    },
    anchor_lang::{prelude::*, AnchorDeserialize},
    anchor_spl::token::TokenAccount,
    std::convert::TryFrom,
};

pub const SEED_REGISTRY: &[u8] = b"registry";

/**
 * Registry
 */

#[account]
#[derive(Debug)]
pub struct Registry {
    pub is_locked: bool,
    pub total_workers: u64,
}

impl Registry {
    pub fn pubkey() -> Pubkey {
        Pubkey::find_program_address(&[SEED_REGISTRY], &crate::ID).0
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
    fn init(&mut self) -> Result<()>;

    fn new_node(
        &mut self,
        authority: &mut Signer,
        node: &mut Account<Node>,
        stake: &mut Account<TokenAccount>,
        worker: &Signer,
    ) -> Result<()>;

    fn lock(&mut self) -> Result<()>;

    fn unlock(&mut self) -> Result<()>;
}

impl RegistryAccount for Account<'_, Registry> {
    fn init(&mut self) -> Result<()> {
        self.is_locked = false;
        self.total_workers = 0;
        Ok(())
    }

    fn new_node(
        &mut self,
        authority: &mut Signer,
        node: &mut Account<Node>,
        stake: &mut Account<TokenAccount>,
        worker: &Signer,
    ) -> Result<()> {
        require!(!self.is_locked, ClockworkError::RegistryLocked);
        node.init(authority, self.total_workers, stake, worker)?;
        self.total_workers = self.total_workers.checked_add(1).unwrap();
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
