use {
    crate::pda::PDA,
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
    pub target_ping: i64,
    pub bump: u8,
}

impl Health {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_HEALTH], &crate::ID)
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
    fn open(&mut self, bump: u8) -> Result<()>;

    fn ping(&mut self, clock: &Sysvar<Clock>) -> Result<()>;

    fn reset(&mut self, clock: &Sysvar<Clock>) -> Result<()>;
}

impl HealthAccount for Account<'_, Health> {
    fn open(&mut self, bump: u8) -> Result<()> {
        self.last_ping = 0;
        self.target_ping = 0;
        self.bump = bump;
        Ok(())
    }

    fn ping(&mut self, clock: &Sysvar<Clock>) -> Result<()> {
        self.last_ping = clock.unix_timestamp;
        self.target_ping = self.target_ping.checked_add(1).unwrap();
        Ok(())
    }

    fn reset(&mut self, clock: &Sysvar<Clock>) -> Result<()> {
        self.last_ping = clock.unix_timestamp;
        self.target_ping = clock.unix_timestamp;
        Ok(())
    }
}
