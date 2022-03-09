use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime};

use {
    super::{Config, Daemon, DaemonAccount, Fee},
    crate::{errors::CronosError, pda::PDA},
    anchor_lang::{prelude::borsh::BorshSchema, prelude::*, AnchorDeserialize},
    chrono::Utc,
    cronos_cron::Schedule,
    solana_program::instruction::Instruction,
    std::convert::TryFrom,
};

pub const SEED_TASK: &[u8] = b"task";

/**
 * Task
 */

#[account]
#[derive(Debug)]
pub struct Task {
    pub daemon: Pubkey,
    pub executor: Pubkey,
    pub exec_at: i64,
    pub int: u128,
    pub ix: InstructionData,
    pub schedule: String,
    pub status: TaskStatus,
    pub bump: u8,
}

impl Task {
    pub fn pda(daemon: Pubkey, id: u128) -> PDA {
        Pubkey::find_program_address(
            &[SEED_TASK, daemon.as_ref(), id.to_be_bytes().as_ref()],
            &crate::ID,
        )
    }
}

impl TryFrom<Vec<u8>> for Task {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Task::try_deserialize(&mut data.as_slice())
    }
}

/**
 * TaskAccount
 */

pub trait TaskAccount {
    fn init(
        &mut self,
        clock: &Sysvar<Clock>,
        daemon: &mut Account<Daemon>,
        executor: Pubkey,
        ix: InstructionData,
        schedule: String,
        bump: u8,
    ) -> Result<()>;

    fn cancel(&mut self) -> Result<()>;

    fn delegate(&mut self, to: Pubkey) -> Result<()>;

    fn execute(
        &mut self,
        account_infos: &[AccountInfo],
        config: &Account<Config>,
        daemon: &mut Account<Daemon>,
        executor: &mut Signer,
        fee: &mut Account<Fee>,
    ) -> Result<()>;

    fn next_exec_after(&self, ts: i64) -> Option<i64>;
}

impl TaskAccount for Account<'_, Task> {
    fn init(
        &mut self,
        clock: &Sysvar<Clock>,
        daemon: &mut Account<Daemon>,
        executor: Pubkey,
        ix: InstructionData,
        schedule: String,
        bump: u8,
    ) -> Result<()> {
        // Reject the instruction if it has other signers besides the daemon.
        for acc in ix.accounts.as_slice() {
            require!(
                !acc.is_signer || acc.pubkey == daemon.key(),
                CronosError::InvalidSignatory
            );
        }

        // Initialize task account.
        self.daemon = daemon.key();
        self.executor = executor;
        self.int = daemon.task_count;
        self.ix = ix;
        self.schedule = schedule;
        self.status = TaskStatus::Queued;
        self.bump = bump;

        // Load exec_at
        self.exec_at = self
            .next_exec_after(clock.unix_timestamp)
            .ok_or(CronosError::InvalidSchedule)
            .unwrap();

        // Increment daemon task count
        daemon.task_count = daemon.task_count.checked_add(1).unwrap();

        Ok(())
    }

    fn cancel(&mut self) -> Result<()> {
        self.status = TaskStatus::Cancelled;
        Ok(())
    }

    fn delegate(&mut self, to: Pubkey) -> Result<()> {
        self.executor = to;
        Ok(())
    }

    fn execute(
        &mut self,
        account_infos: &[AccountInfo],
        config: &Account<Config>,
        daemon: &mut Account<Daemon>,
        executor: &mut Signer,
        fee: &mut Account<Fee>,
    ) -> Result<()> {
        // Invoke instruction.
        daemon.invoke(&Instruction::from(&self.ix), account_infos)?;

        // Increment collectable fee balance.
        fee.balance = fee.balance.checked_add(config.program_fee).unwrap();

        // Transfer lamports from daemon to fee account.
        **daemon.to_account_info().try_borrow_mut_lamports()? = daemon
            .to_account_info()
            .lamports()
            .checked_sub(config.program_fee)
            .unwrap();
        **fee.to_account_info().try_borrow_mut_lamports()? = fee
            .to_account_info()
            .lamports()
            .checked_add(config.program_fee)
            .unwrap();

        // Transfer lamports from daemon to worker.
        **daemon.to_account_info().try_borrow_mut_lamports()? = daemon
            .to_account_info()
            .lamports()
            .checked_sub(config.program_fee)
            .unwrap();
        **executor.to_account_info().try_borrow_mut_lamports()? = executor
            .to_account_info()
            .lamports()
            .checked_add(config.program_fee)
            .unwrap();

        // Update task schedule.
        match self.next_exec_after(self.exec_at) {
            Some(next_exec_at) => self.exec_at = next_exec_at,
            None => self.status = TaskStatus::Done, // TODO close the task account if done
        };

        Ok(())
    }

    fn next_exec_after(&self, ts: i64) -> Option<i64> {
        match Schedule::from_str(&self.schedule)
            .unwrap()
            .after(&DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(ts, 0),
                Utc,
            ))
            .take(1)
            .next()
        {
            Some(datetime) => Some(datetime.timestamp()),
            None => None,
        }
    }
}

/**
 * InstructionData
 */

#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct InstructionData {
    /// Pubkey of the instruction processor that executes this instruction
    pub program_id: Pubkey,
    /// Metadata for what accounts should be passed to the instruction processor
    pub accounts: Vec<AccountMetaData>,
    /// Opaque data passed to the instruction processor
    pub data: Vec<u8>,
}

impl From<Instruction> for InstructionData {
    fn from(instruction: Instruction) -> Self {
        InstructionData {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(|a| AccountMetaData {
                    pubkey: a.pubkey,
                    is_signer: a.is_signer,
                    is_writable: a.is_writable,
                })
                .collect(),
            data: instruction.data,
        }
    }
}

impl From<&InstructionData> for Instruction {
    fn from(instruction: &InstructionData) -> Self {
        Instruction {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(|a| AccountMeta {
                    pubkey: a.pubkey,
                    is_signer: a.is_signer,
                    is_writable: a.is_writable,
                })
                .collect(),
            data: instruction.data.clone(),
        }
    }
}

impl TryFrom<Vec<u8>> for InstructionData {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Ok(
            borsh::try_from_slice_with_schema::<InstructionData>(data.as_slice())
                .map_err(|_err| ErrorCode::AccountDidNotDeserialize)?,
        )
    }
}

/**
 * AccountMetaData
 */

#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct AccountMetaData {
    /// An account's public key
    pub pubkey: Pubkey,
    /// True if an Instruction requires a Transaction signature matching `pubkey`.
    pub is_signer: bool,
    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,
}

/**
 * TaskSchedule
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug)]
pub struct TaskSchedule {
    pub exec_at: i64, // Time to execute at
    pub stop_at: i64, // Stop executing at
    pub recurr: i64,  // Duration between exec
}

/**
 * TaskStatus
 */

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum TaskStatus {
    Cancelled,
    Done,
    Queued,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Cancelled => write!(f, "cancelled"),
            TaskStatus::Done => write!(f, "done"),
            TaskStatus::Queued => write!(f, "queued"),
        }
    }
}
