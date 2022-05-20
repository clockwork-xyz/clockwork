use {
    crate::pda::PDA,
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
    pub delegate_fee: u64,
    pub delegate_holdout_period: i64,
    pub delegate_spam_penalty: u64,
    pub program_fee: u64,
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
    pub delegate_fee: u64,
    pub delegate_holdout_period: i64,
    pub delegate_spam_penalty: u64,
    pub program_fee: u64,
}
/**
 * ConfigAccount
 */

pub trait ConfigAccount {
    fn new(&mut self, admin: Pubkey) -> Result<()>;

    fn update(&mut self, settings: ConfigSettings) -> Result<()>;
}

impl ConfigAccount for Account<'_, Config> {
    fn new(&mut self, admin: Pubkey) -> Result<()> {
        self.admin = admin;
        self.delegate_fee = 0;
        self.delegate_holdout_period = 0;
        self.delegate_spam_penalty = 0;
        self.program_fee = 0;
        Ok(())
    }

    fn update(&mut self, settings: ConfigSettings) -> Result<()> {
        self.admin = settings.admin;
        self.delegate_fee = settings.delegate_fee;
        self.delegate_holdout_period = settings.delegate_holdout_period;
        self.delegate_spam_penalty = settings.delegate_spam_penalty;
        self.program_fee = settings.program_fee;
        Ok(())
    }
}
