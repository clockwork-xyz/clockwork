use {
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
    pub current_epoch_id: u64,
    pub locked: bool,
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

    fn lock(&mut self) -> Result<()>;

    fn unlock(&mut self) -> Result<()>;
}

impl RegistryAccount for Account<'_, Registry> {
    fn init(&mut self) -> Result<()> {
        self.current_epoch_id = 0;
        self.locked = false;
        self.total_workers = 0;
        Ok(())
    }

    fn lock(&mut self) -> Result<()> {
        self.locked = true;
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        self.locked = false;
        Ok(())
    }
}
