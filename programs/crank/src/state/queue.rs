use super::Exec;

use {
    super::InstructionData,
    crate::errors::ClockworkError,
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
    pub authority: Pubkey,
    pub exec_count: u64,
    pub instruction: InstructionData,
    pub last_exec: Option<Pubkey>,
    pub name: String,
    pub trigger: Trigger,
}

impl Queue {
    pub fn pubkey(authority: Pubkey, name: String) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_QUEUE, authority.as_ref(), name.as_bytes()],
            &crate::ID,
        )
        .0
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
    fn init(
        &mut self,
        authority: Pubkey,
        instruction: InstructionData,
        name: String,
        trigger: Trigger,
    ) -> Result<()>;

    fn crank(
        &self,
        account_infos: &[AccountInfo],
        bump: u8,
        exec: &mut Account<Exec>,
        ix: &InstructionData,
    ) -> Result<()>;
}

impl QueueAccount for Account<'_, Queue> {
    fn init(
        &mut self,
        authority: Pubkey,
        instruction: InstructionData,
        name: String,
        trigger: Trigger,
    ) -> Result<()> {
        self.authority = authority.key();
        self.exec_count = 0;
        self.instruction = instruction;
        self.last_exec = None;
        self.name = name;
        self.trigger = trigger;
        Ok(())
    }

    fn crank(
        &self,
        account_infos: &[AccountInfo],
        bump: u8,
        exec: &mut Account<Exec>,
        instruction: &InstructionData,
    ) -> Result<()> {
        // Invoke the provided instruction
        invoke_signed(
            &Instruction::from(instruction),
            account_infos,
            &[&[
                SEED_QUEUE,
                self.authority.as_ref(),
                self.name.as_bytes(),
                &[bump],
            ]],
        )
        .map_err(|_err| ClockworkError::InnerIxFailed)?;

        // Parse the crank response
        match get_return_data() {
            None => {
                exec.instruction = None;
            }
            Some((program_id, return_data)) => {
                if program_id != instruction.program_id {
                    return Err(ClockworkError::InvalidReturnData.into());
                } else {
                    let crank_response = CrankResponse::try_from_slice(return_data.as_slice())
                        .map_err(|_err| ClockworkError::InvalidCrankResponse)?;
                    exec.instruction = crank_response.next_instruction;
                }
            }
        };

        Ok(())
    }
}

/**
 * CrankResponse
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct CrankResponse {
    pub next_instruction: Option<InstructionData>,
}

impl Default for CrankResponse {
    fn default() -> Self {
        return Self {
            next_instruction: None,
        };
    }
}

/**
 * Trigger
 */

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub enum Trigger {
    Cron { schedule: String },
    Immediate,
}
