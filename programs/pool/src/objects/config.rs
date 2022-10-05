use anchor_lang::{prelude::*, AnchorDeserialize};

pub const SEED_CONFIG: &[u8] = b"config";

/**
 * Config
 */

#[account]
#[derive(Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub pool_authority: Pubkey,
}

impl Config {
    pub fn pubkey() -> Pubkey {
        Pubkey::find_program_address(&[SEED_CONFIG], &crate::ID).0
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
    pub pool_authority: Pubkey,
}

/**
 * ConfigAccount
 */

pub trait ConfigAccount {
    fn init(&mut self, admin: Pubkey, pool_authority: Pubkey) -> Result<()>;

    fn update(&mut self, settings: ConfigSettings) -> Result<()>;
}

impl ConfigAccount for Account<'_, Config> {
    fn init(&mut self, admin: Pubkey, pool_authority: Pubkey) -> Result<()> {
        self.admin = admin;
        self.pool_authority = pool_authority;
        Ok(())
    }

    fn update(&mut self, settings: ConfigSettings) -> Result<()> {
        self.admin = settings.admin;
        self.pool_authority = settings.pool_authority;
        Ok(())
    }
}
