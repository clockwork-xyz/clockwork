use super::InstructionData;

use {
    super::Queue,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_EXEC: &[u8] = b"exec";

/**
 * Exec
 */

#[account]
#[derive(Debug)]
pub struct Exec {
    pub context: ExecContext,
    pub id: u64,
    pub instruction: Option<InstructionData>,
    pub queue: Pubkey,
}

impl Exec {
    pub fn pubkey(id: u64, queue: Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_EXEC, queue.as_ref(), id.to_be_bytes().as_ref()],
            &crate::ID,
        )
        .0
    }
}

impl TryFrom<Vec<u8>> for Exec {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Exec::try_deserialize(&mut data.as_slice())
    }
}

/**
 * ExecAccount
 */

pub trait ExecAccount {
    fn init(&mut self, context: ExecContext, queue: &mut Account<Queue>) -> Result<()>;
}

impl ExecAccount for Account<'_, Exec> {
    fn init(&mut self, context: ExecContext, queue: &mut Account<Queue>) -> Result<()> {
        self.context = context;
        self.id = queue.exec_count;
        self.instruction = None;
        self.queue = queue.key();

        queue.exec_count = queue.exec_count.checked_add(1).unwrap();

        Ok(())
    }
}

/**
 * ExecContext
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub enum ExecContext {
    Cron { unix_timestamp: i64 },
    Immediate,
}
