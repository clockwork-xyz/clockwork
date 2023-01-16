use anchor_lang::{prelude::*, AnchorDeserialize};
use clockwork_macros::TryFromData;

pub const SEED_CONFIG: &[u8] = b"config";

/**
 * Defaults
 */

static DEFAULT_REQUEST_FEE: u64 = 1_000_000; // 0.001 SOL
static DEFAULT_TIMEOUT_THRESHOLD: u64 = 100; // 100 slots

/**
 * Config
 */

#[account]
#[derive(Debug, TryFromData)]
pub struct Config {
    pub admin: Pubkey,
    pub request_fee: u64, // Amount to charge per request and payout to workers
    pub timeout_threshold: u64, // Duration (slots) to wait before a requests is considered "timed out"
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
    pub request_fee: u64,
    pub timeout_threshold: u64,
}

/**
 * ConfigAccount
 */

pub trait ConfigAccount {
    fn init(&mut self, admin: Pubkey) -> Result<()>;

    fn update(&mut self, settings: ConfigSettings) -> Result<()>;
}

impl ConfigAccount for Account<'_, Config> {
    fn init(&mut self, admin: Pubkey) -> Result<()> {
        self.admin = admin;
        self.request_fee = DEFAULT_REQUEST_FEE;
        self.timeout_threshold = DEFAULT_TIMEOUT_THRESHOLD;
        Ok(())
    }

    fn update(&mut self, settings: ConfigSettings) -> Result<()> {
        self.admin = settings.admin;
        self.request_fee = settings.request_fee;
        self.timeout_threshold = settings.timeout_threshold;
        Ok(())
    }
}
