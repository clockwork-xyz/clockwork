use {
    super::{BytesSize, ClockData, InstructionData},
    crate::errors::ClockworkError,
    anchor_lang::{
        prelude::*,
        solana_program::{
            instruction::Instruction,
            program::{get_return_data, invoke_signed},
        },
        AnchorDeserialize, AnchorSerialize,
    },
    std::{
        convert::TryFrom,
        hash::{Hash, Hasher},
        mem::{size_of, size_of_val},
    },
};

pub const SEED_QUEUE: &[u8] = b"queue";

/**
 * Queue
 */

#[account]
#[derive(Debug)]
pub struct Queue {
    pub authority: Pubkey,
    pub created_at: ClockData,
    pub exec_context: Option<ExecContext>,
    pub first_instruction: InstructionData,
    pub name: String,
    pub next_instruction: Option<InstructionData>,
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

impl Hash for Queue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.authority.hash(state);
        self.name.hash(state);
    }
}

impl PartialEq for Queue {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.name.eq(&other.name)
    }
}

impl Eq for Queue {}

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

    fn crank(&mut self, account_infos: &[AccountInfo], bump: u8, worker: &Signer) -> Result<()>;
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
        self.created_at = Clock::get().unwrap().into();
        self.exec_context = None;
        self.first_instruction = instruction;
        self.name = name;
        self.next_instruction = None;
        self.trigger = trigger;
        Ok(())
    }

    fn crank(&mut self, account_infos: &[AccountInfo], bump: u8, worker: &Signer) -> Result<()> {
        // Get the instruction to crank
        let first_instruction: &InstructionData = &self.clone().first_instruction;
        let next_instruction: &Option<InstructionData> = &self.clone().next_instruction;
        let instruction = next_instruction.as_ref().unwrap_or(first_instruction);

        // Inject the worker's pubkey for the Clockwork payer ID
        let normalized_accounts: &mut Vec<AccountMeta> = &mut vec![];
        instruction.accounts.iter().for_each(|acc| {
            let acc_pubkey = if acc.pubkey == crate::payer::ID {
                worker.key()
            } else {
                acc.pubkey
            };
            normalized_accounts.push(AccountMeta {
                pubkey: acc_pubkey,
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            });
        });

        // Invoke the provided instruction
        invoke_signed(
            &Instruction {
                program_id: instruction.program_id,
                data: instruction.data.clone(),
                accounts: normalized_accounts.to_vec(),
            },
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
                self.next_instruction = None;
            }
            Some((program_id, return_data)) => {
                require!(
                    program_id.eq(&instruction.program_id),
                    ClockworkError::InvalidReturnData
                );
                let crank_response = CrankResponse::try_from_slice(return_data.as_slice())
                    .map_err(|_err| ClockworkError::InvalidCrankResponse)?;
                self.next_instruction = crank_response.next_instruction;
            }
        };

        // Realloc the queue account
        let new_size = 8
            + size_of::<Queue>()
            + size_of_val(&self.exec_context)
            + size_of_val(&self.name)
            + size_of_val(&self.trigger)
            + self.first_instruction.bytes_size()
            + match self.next_instruction.clone() {
                None => size_of_val(&self.next_instruction),
                Some(next_instruction) => next_instruction.bytes_size(),
            };
        self.to_account_info().realloc(new_size, false)?;

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

/**
 * ExecContext
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ExecContext {
    Cron { last_exec_at: i64 },
    Immediate,
}
