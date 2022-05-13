use super::InstructionData;

use {
    crate::{errors::CronosError, pda::PDA, responses::ExecResponse},
    anchor_lang::{
        prelude::*,
        solana_program::{
            instruction::Instruction,
            program::{get_return_data, invoke_signed},
        },
        AnchorDeserialize,
    },
    std::convert::TryFrom,
};

pub const SEED_QUEUE: &[u8] = b"queue";

/**
 * Queue
 */

#[account]
#[derive(Debug)]
pub struct Queue {
    pub owner: Pubkey,
    pub task_count: u128,
    pub bump: u8,
}

impl Queue {
    pub fn pda(owner: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_QUEUE, owner.as_ref()], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Queue {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Queue::try_deserialize(&mut data.as_slice())
    }
}

/**
 * QueueAccount
 */

pub trait QueueAccount {
    fn new(&mut self, bump: u8, owner: Pubkey) -> Result<()>;

    fn process(&self, ix: &InstructionData, account_infos: &[AccountInfo]) -> Result<ExecResponse>;
}

impl QueueAccount for Account<'_, Queue> {
    fn new(&mut self, bump: u8, owner: Pubkey) -> Result<()> {
        self.bump = bump;
        self.owner = owner;
        self.task_count = 0;
        Ok(())
    }

    fn process(&self, ix: &InstructionData, account_infos: &[AccountInfo]) -> Result<ExecResponse> {
        invoke_signed(
            &Instruction::from(ix),
            account_infos,
            &[&[SEED_QUEUE, self.owner.as_ref(), &[self.bump]]],
        )
        .map_err(|_err| CronosError::InnerIxFailed)?;

        let exec_response = get_return_data()
            .ok_or(CronosError::InvalidExecResponse)
            .and_then(|(program_id, return_data)| {
                (program_id == ix.program_id)
                    .then(|| return_data)
                    .ok_or(CronosError::InvalidExecResponse)
            })
            .map(|return_data| {
                ExecResponse::try_from_slice(return_data.as_slice())
                    .map_err(|_err| CronosError::InvalidExecResponse)
            })?;

        Ok(exec_response?)
    }
}
