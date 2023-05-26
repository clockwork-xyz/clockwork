use std::mem::size_of;

use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use clockwork_utils::thread::{ClockData, SerializableInstruction, Trigger};

pub use clockwork_utils::thread::Equality;

pub const SEED_THREAD: &[u8] = b"thread";

/// Static space for next_instruction field.
pub const NEXT_INSTRUCTION_SIZE: usize = 1232;

/// Tracks the current state of a transaction thread on Solana.
#[account]
#[derive(Debug)]
pub struct Thread {
    /// The owner of this thread.
    pub authority: Pubkey,
    /// The bump, used for PDA validation.
    pub bump: u8,
    /// The cluster clock at the moment the thread was created.
    pub created_at: ClockData,
    /// The context of the thread's current execution state.
    pub exec_context: Option<ExecContext>,
    /// The number of lamports to payout to workers per execution.
    pub fee: u64,
    /// The id of the thread, given by the authority.
    pub id: Vec<u8>,
    /// The instructions to be executed.
    pub instructions: Vec<SerializableInstruction>,
    /// The name of the thread.
    pub name: String,
    /// The next instruction to be executed.
    pub next_instruction: Option<SerializableInstruction>,
    /// Whether or not the thread is currently paused.
    pub paused: bool,
    /// The maximum number of execs allowed per slot.
    pub rate_limit: u64,
    /// The triggering event to kickoff a thread.
    pub trigger: Trigger,
}

impl Thread {
    /// Derive the pubkey of a thread account.
    pub fn pubkey(authority: Pubkey, id: Vec<u8>) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_THREAD, authority.as_ref(), id.as_slice()],
            &crate::ID,
        )
        .0
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.id.eq(&other.id)
    }
}

impl Eq for Thread {}

/// Trait for reading and writing to a thread account.
pub trait ThreadAccount {
    /// Get the pubkey of the thread account.
    fn pubkey(&self) -> Pubkey;

    /// Allocate more memory for the account.
    fn realloc(&mut self) -> Result<()>;
}

impl ThreadAccount for Account<'_, Thread> {
    fn pubkey(&self) -> Pubkey {
        Thread::pubkey(self.authority, self.id.clone())
    }

    fn realloc(&mut self) -> Result<()> {
        // Realloc memory for the thread account
        let data_len = vec![
            8,
            size_of::<Thread>(),
            self.id.len(),
            self.instructions.try_to_vec()?.len(),
            self.trigger.try_to_vec()?.len(),
            NEXT_INSTRUCTION_SIZE,
        ]
        .iter()
        .sum();
        self.to_account_info().realloc(data_len, false)?;
        Ok(())
    }
}

/// The execution context of a particular transaction thread.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecContext {
    /// Index of the next instruction to be executed.
    pub exec_index: u64,

    /// Number of execs since the last tx reimbursement.
    /// To be deprecated in v3 since we now reimburse for every transaction.
    pub execs_since_reimbursement: u64,

    /// Number of execs in this slot.
    pub execs_since_slot: u64,

    /// Slot of the last exec
    pub last_exec_at: u64,

    /// Context for the triggering condition
    pub trigger_context: TriggerContext,
}

/// The event which allowed a particular transaction thread to be triggered.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerContext {
    /// A running hash of the observed account data.
    Account {
        /// The account's data hash.
        data_hash: u64,
    },

    /// A cron execution context.
    Cron {
        /// The threshold moment the schedule was waiting for.
        started_at: i64,
    },

    /// The trigger context for threads with a "now" trigger.
    Now,

    /// The trigger context for threads with a "slot" trigger.
    Slot {
        /// The threshold slot the schedule was waiting for.
        started_at: u64,
    },

    /// The trigger context for threads with an "epoch" trigger.
    Epoch {
        /// The threshold epoch the schedule was waiting for.
        started_at: u64,
    },

    /// The trigger context for threads with an "timestamp" trigger.
    Timestamp {
        /// The threshold moment the schedule was waiting for.
        started_at: i64,
    },

    /// The trigger context for threads with a "pyth" trigger.
    Pyth { price: i64 },
}

/// The properties of threads which are updatable.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ThreadSettings {
    pub fee: Option<u64>,
    pub instructions: Option<Vec<SerializableInstruction>>,
    pub name: Option<String>,
    pub rate_limit: Option<u64>,
    pub trigger: Option<Trigger>,
}
