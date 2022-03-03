use {
    crate::pda::PDA,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_WORKER_PAGE: &[u8] = b"worker_page";

/**
 * WorkerPage
 */

#[account]
#[derive(Debug)]
pub struct WorkerPage {
    pub bump: u8,
}

impl WorkerPage {
    pub fn pda() -> PDA {
        Pubkey::find_program_address(&[SEED_WORKER_PAGE], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for WorkerPage {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        WorkerPage::try_deserialize(&mut data.as_slice())
    }
}

/**
 * WorkerPageAccount
 */

pub trait WorkerPageAccount {
    fn init(&mut self, bump: u8) -> Result<()>;
}

impl WorkerPageAccount for Account<'_, WorkerPage> {
    fn init(&mut self, bump: u8) -> Result<()> {
        self.bump = bump;
        Ok(())
    }
}
