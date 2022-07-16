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
    pub program_fee: u64,
    pub worker_fee: u64,
    pub worker_holdout_period: i64,
    pub worker_spam_penalty: u64,
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
    pub program_fee: u64,
    pub worker_fee: u64,
    pub worker_holdout_period: i64,
    pub worker_spam_penalty: u64,
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
        self.program_fee = 0;
        self.worker_fee = 0;
        self.worker_holdout_period = 0;
        self.worker_spam_penalty = 0;
        Ok(())
    }

    fn update(&mut self, settings: ConfigSettings) -> Result<()> {
        self.admin = settings.admin;
        self.program_fee = settings.program_fee;
        self.worker_fee = settings.worker_fee;
        self.worker_holdout_period = settings.worker_holdout_period;
        self.worker_spam_penalty = settings.worker_spam_penalty;
        Ok(())
    }
}
