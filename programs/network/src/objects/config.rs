use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_CONFIG: &[u8] = b"config";

static DEFAULT_SLOTS_PER_ROTATION: u64 = 10;

/**
 * Config
 */

#[account]
#[derive(Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub slots_per_rotation: u64, // Target number of slots between each rotation
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
    pub mint: Pubkey,
    pub slots_per_rotation: u64,
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
        self.slots_per_rotation = DEFAULT_SLOTS_PER_ROTATION;
        Ok(())
    }

    fn update(&mut self, settings: ConfigSettings) -> Result<()> {
        self.admin = settings.admin;
        self.mint = settings.mint;
        self.slots_per_rotation = settings.slots_per_rotation;
        Ok(())
    }
}
