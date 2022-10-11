use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::{
        collections::hash_map::DefaultHasher,
        convert::TryFrom,
        hash::{Hash, Hasher},
    },
};

pub const SEED_REGISTRY: &[u8] = b"registry";

/// Registry

#[account]
#[derive(Debug)]
pub struct Registry {
    pub current_epoch: u64,
    pub locked: bool,
    pub nonce: u64,
    pub total_pools: u64,
    pub total_unstakes: u64,
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

    fn hash_nonce(&mut self) -> Result<()>;
}

impl RegistryAccount for Account<'_, Registry> {
    fn init(&mut self) -> Result<()> {
        self.current_epoch = 0;
        self.locked = false;
        self.total_workers = 0;
        Ok(())
    }

    fn hash_nonce(&mut self) -> Result<()> {
        let mut hasher = DefaultHasher::new();
        Clock::get().unwrap().slot.hash(&mut hasher);
        self.nonce.hash(&mut hasher);
        self.nonce = hasher.finish();
        Ok(())
    }
}
