use anchor_lang::{prelude::*, AnchorDeserialize};
use clockwork_macros::TryFromData;

pub const SEED_CONFIG: &[u8] = b"config";

/**
 * Config
 */

#[account]
#[derive(Debug, TryFromData)]
pub struct Config {
    pub admin: Pubkey,
    pub epoch_automation: Pubkey,
    pub hasher_automation: Pubkey,
    pub mint: Pubkey,
}

impl Config {
    pub fn pubkey() -> Pubkey {
        Pubkey::find_program_address(&[SEED_CONFIG], &crate::ID).0
    }
}

/**
 * ConfigSettings
 */

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConfigSettings {
    pub admin: Pubkey,
    pub epoch_automation: Pubkey,
    pub hasher_automation: Pubkey,
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
        self.epoch_automation = settings.epoch_automation;
        self.hasher_automation = settings.hasher_automation;
        self.mint = settings.mint;
        Ok(())
    }
}
