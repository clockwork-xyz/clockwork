use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_HEALTH: &[u8] = b"health";

/**
 * Health
 */

#[account]
#[derive(Debug)]
pub struct Health {
    pub last_ping: i64,
}

impl Health {
    pub fn pubkey() -> Pubkey {
        Pubkey::find_program_address(&[SEED_HEALTH], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Health {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Health::try_deserialize(&mut data.as_slice())
    }
}

/**
 * HealthAccount
 */

pub trait HealthAccount {
    fn new(&mut self) -> Result<()>;

    fn ping(&mut self, clock: &Sysvar<Clock>) -> Result<()>;

    fn reset(&mut self, clock: &Sysvar<Clock>) -> Result<()>;
}

impl HealthAccount for Account<'_, Health> {
    fn new(&mut self) -> Result<()> {
        self.last_ping = 0;
        Ok(())
    }

    fn ping(&mut self, clock: &Sysvar<Clock>) -> Result<()> {
        self.last_ping = clock.unix_timestamp;
        Ok(())
    }

    fn reset(&mut self, clock: &Sysvar<Clock>) -> Result<()> {
        self.last_ping = clock.unix_timestamp;
        Ok(())
    }
}
