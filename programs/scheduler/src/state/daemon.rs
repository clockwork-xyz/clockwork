use {
    crate::{errors::CronosError, pda::PDA},
    anchor_lang::{prelude::*, AnchorDeserialize},
    solana_program::{instruction::Instruction, program::invoke_signed},
    std::convert::TryFrom,
};

pub const SEED_DAEMON: &[u8] = b"daemon";

/**
 * Daemon
 */

#[account]
#[derive(Debug)]
pub struct Daemon {
    pub owner: Pubkey,
    pub task_count: u128,
    pub bump: u8,
}

impl Daemon {
    pub fn pda(owner: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_DAEMON, owner.as_ref()], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Daemon {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Daemon::try_deserialize(&mut data.as_slice())
    }
}

/**
 * DaemonAccount
 */

pub trait DaemonAccount {
    fn new(&mut self, owner: Pubkey, bump: u8) -> Result<()>;

    fn sign(&mut self, ix: &Instruction, account_infos: &[AccountInfo]) -> Result<()>;
}

impl DaemonAccount for Account<'_, Daemon> {
    fn new(&mut self, owner: Pubkey, bump: u8) -> Result<()> {
        self.owner = owner;
        self.task_count = 0;
        self.bump = bump;
        Ok(())
    }

    fn sign(&mut self, ix: &Instruction, account_infos: &[AccountInfo]) -> Result<()> {
        invoke_signed(
            ix,
            account_infos,
            &[&[SEED_DAEMON, self.owner.as_ref(), &[self.bump]]],
        )
        .map_err(|_err| CronosError::TaskFailed.into())
    }
}
