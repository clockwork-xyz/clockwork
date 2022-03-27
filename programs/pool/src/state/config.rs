use {
    crate::{errors::{AccountError, AdminError}, pda::PDA},
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_CONFIG: &[u8] = b"config";

/**
 * Config
 */

#[account]
#[derive(Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub bump: u8,
    pub pool_size: usize,
}

impl Config {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_CONFIG], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Config {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Config::try_deserialize(&mut data.as_slice())
    }
}

/**
 * ConfigSettings
 */

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConfigSettings {
    pub admin: Pubkey,
    pub pool_size: usize,
}
/**
 * ConfigAccount
 */

pub trait ConfigAccount {
    fn new(&mut self, admin: Pubkey, bump: u8) -> Result<()>;

    fn update(&mut self, admin: &Signer, settings: ConfigSettings) -> Result<()>;
}

impl ConfigAccount for Account<'_, Config> {
    fn new(&mut self, admin: Pubkey, bump: u8) -> Result<()> {
        require!(self.bump == 0, AccountError::AlreadyInitialized);
        self.admin = admin;
        self.bump = bump;
        self.pool_size = 1; 
        Ok(())
    }

    fn update(&mut self, admin: &Signer, settings: ConfigSettings) -> Result<()> {
        require!(self.admin == admin.key(), AdminError::NotAuthorized);
        self.admin = settings.admin;
        self.pool_size = settings.pool_size;
        Ok(())
    }
}
