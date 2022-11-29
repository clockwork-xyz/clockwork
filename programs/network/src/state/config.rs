use {
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
    pub epoch_thread: Pubkey,
    pub hasher_thread: Pubkey,
    pub mint: Pubkey,
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
    pub epoch_thread: Pubkey,
    pub hasher_thread: Pubkey,
    pub mint: Pubkey,
}

/**
 * ConfigAccount
 */

pub trait ConfigAccount {
    fn init(&mut self, admin: Pubkey, mint: Pubkey) -> Result<()>;

    fn update(&mut self, settings: ConfigSettings) -> Result<()>;
}

impl ConfigAccount for Account<'_, Config> {
    fn init(&mut self, admin: Pubkey, mint: Pubkey) -> Result<()> {
        self.admin = admin;
        self.mint = mint;
        Ok(())
    }

    fn update(&mut self, settings: ConfigSettings) -> Result<()> {
        self.admin = settings.admin;
        self.epoch_thread = settings.epoch_thread;
        self.hasher_thread = settings.hasher_thread;
        self.mint = settings.mint;
        Ok(())
    }
}
