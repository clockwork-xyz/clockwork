use {
    crate::pda::PDA,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_HEARTBEAT: &[u8] = b"heartbeat";

/**
 * Heartbeat
 */

#[account]
#[derive(Debug)]
pub struct Heartbeat {
    pub last_ping: i64,
    pub target_ping: i64,
}

impl Heartbeat {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_HEARTBEAT], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Heartbeat {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Heartbeat::try_deserialize(&mut data.as_slice())
    }
}

/**
 * HeartbeatAccount
 */

pub trait HeartbeatAccount {
    fn new(&mut self) -> Result<()>;

    fn ping(&mut self, clock: &Sysvar<Clock>) -> Result<()>;

    fn reset(&mut self, clock: &Sysvar<Clock>) -> Result<()>;
}

impl HeartbeatAccount for Account<'_, Heartbeat> {
    fn new(&mut self) -> Result<()> {
        self.last_ping = 0;
        self.target_ping = 0;
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
