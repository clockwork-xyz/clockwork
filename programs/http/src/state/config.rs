use {
    crate::errors::CronosError,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_CONFIG: &[u8] = b"config";

static DEFAULT_REQUEST_FEE: u64 = 1_000_000; // Default 0.001 SOL per request

/**
 * Config
 */

#[account]
#[derive(Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub request_fee: u64,
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
    pub request_fee: u64,
}

/**
 * ConfigAccount
 */

pub trait ConfigAccount {
    fn new(&mut self, admin: Pubkey) -> Result<()>;

    fn update(&mut self, admin: &Signer, settings: ConfigSettings) -> Result<()>;
}

impl ConfigAccount for Account<'_, Config> {
    fn new(&mut self, admin: Pubkey) -> Result<()> {
        self.admin = admin;
        self.request_fee = DEFAULT_REQUEST_FEE;
        Ok(())
    }

    fn update(&mut self, admin: &Signer, settings: ConfigSettings) -> Result<()> {
        require!(
            self.admin == admin.key(),
            CronosError::AdminAuthorityInvalid
        );
        self.admin = settings.admin;
        self.request_fee = settings.request_fee;
        Ok(())
    }
}
