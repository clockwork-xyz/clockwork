use {
    super::{ClockData, InstructionData},
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
    pub id: String,
    pub is_paused: bool,
    pub kickoff_instruction: InstructionData,
    pub next_instruction: Option<InstructionData>,
    pub trigger: Trigger,
}

impl Queue {
    pub fn pubkey(authority: Pubkey, id: String) -> Pubkey {
        Pubkey::find_program_address(&[SEED_QUEUE, authority.as_ref(), id.as_bytes()], &crate::ID).0
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
        self.id.hash(state);
    }
}

impl PartialEq for Queue {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.id.eq(&other.id)
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
        id: String,
        kickoff_instruction: InstructionData,
        trigger: Trigger,
    ) -> Result<()>;

    fn crank(&mut self, account_infos: &[AccountInfo], bump: u8, worker: &Signer) -> Result<()>;

    fn realloc(&mut self) -> Result<()>;
}

impl QueueAccount for Account<'_, Queue> {
    fn init(
        &mut self,
        authority: Pubkey,
        id: String,
        kickoff_instruction: InstructionData,
        trigger: Trigger,
    ) -> Result<()> {
        self.authority = authority.key();
        self.created_at = Clock::get().unwrap().into();
        self.exec_context = None;
        self.id = id;
        self.is_paused = false;
        self.kickoff_instruction = kickoff_instruction;
        self.next_instruction = None;
        self.trigger = trigger;
        Ok(())
    }

    fn crank(&mut self, account_infos: &[AccountInfo], bump: u8, worker: &Signer) -> Result<()> {
        // Record the worker's lamports before invoking inner ixs
        let worker_lamports_pre = worker.lamports();

        // Get the instruction to crank
        let kickoff_instruction: &InstructionData = &self.clone().kickoff_instruction;
        let next_instruction: &Option<InstructionData> = &self.clone().next_instruction;
        let instruction = next_instruction.as_ref().unwrap_or(kickoff_instruction);

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
                self.id.as_bytes(),
                &[bump],
            ]],
        )?;

        // Verify that the inner ix did not write data to the worker address
        require!(worker.data_is_empty(), ClockworkError::UnauthorizedWrite);

        // Parse the crank response
        match get_return_data() {
            None => {
                self.next_instruction = None;
            }
            Some((program_id, return_data)) => {
                require!(
                    program_id.eq(&instruction.program_id),
                    ClockworkError::InvalidCrankResponse
                );
                let crank_response = CrankResponse::try_from_slice(return_data.as_slice())
                    .map_err(|_err| ClockworkError::InvalidCrankResponse)?;
                self.next_instruction = crank_response.next_instruction;
            }
        };

        // Realloc the queue account
        self.realloc()?;

        // Reimbursement worker for lamports paid during inner ix
        let worker_lamports_post = worker.lamports();
        let worker_reimbursement = worker_lamports_pre
            .checked_sub(worker_lamports_post)
            .unwrap();
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(worker_reimbursement)
            .unwrap();
        **worker.to_account_info().try_borrow_mut_lamports()? = worker
            .to_account_info()
            .lamports()
            .checked_add(worker_reimbursement)
            .unwrap();

        Ok(())
    }

    fn realloc(&mut self) -> Result<()> {
        // Realloc memory for the queue account
        let data_len = 8 + self.try_to_vec()?.len();
        self.to_account_info().realloc(data_len, false)?;
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
    Account { pubkey: Pubkey },
    Cron { schedule: String },
    Immediate,
}

/**
 * ExecContext
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ExecContext {
    Account { data_hash: u64 },
    Cron { started_at: i64 },
    Immediate,
}
