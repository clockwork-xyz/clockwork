use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_CONFIG: &[u8] = b"config";

static DEFAULT_CRANK_FEE: u64 = 1_000;

/// The config object, recording the config values of a specific Clockwork queue program deployment.
#[account]
#[derive(Debug)]
pub struct Config {
    /// The admin of the deployed program. This value is initialized to whomever calls `initialize`
    /// first after deployment. The admin may redelegate authority to another account.
    pub admin: Pubkey,

    /// The fee paid out to workers by users per successful crank.
    pub crank_fee: u64,

    /// The public address of the worker pool.
    pub worker_pool: Pubkey,
}

impl Config {
    /// Derive the pubkey of the config account.
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

/// The mutable config settings.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConfigSettings {
    /// The admin authority of the deployed program.
    pub admin: Pubkey,
    /// The fee paid out to workers by users per successful crank.
    pub crank_fee: u64,
    /// The public address of the worker pool.
    pub worker_pool: Pubkey,
}

/// Trait for reading and writing to the config account.
pub trait ConfigAccount {
    /// Initialize the account to hold config object.
    fn init(&mut self, admin: Pubkey, worker_pool: Pubkey) -> Result<()>;

    /// Updates the config object.
    fn update(&mut self, settings: ConfigSettings) -> Result<()>;
}

impl ConfigAccount for Account<'_, Config> {
    fn init(&mut self, admin: Pubkey, worker_pool: Pubkey) -> Result<()> {
        self.admin = admin;
        self.crank_fee = DEFAULT_CRANK_FEE;
        self.worker_pool = worker_pool;
        Ok(())
    }

    fn update(&mut self, settings: ConfigSettings) -> Result<()> {
        self.admin = settings.admin;
        self.crank_fee = settings.crank_fee;
        self.worker_pool;
        Ok(())
    }
}
