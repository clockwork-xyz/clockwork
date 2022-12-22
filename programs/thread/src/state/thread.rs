use {
    crate::errors::ClockworkError,
    crate::state::*,
    anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize},
    std::convert::TryFrom,
};

pub const SEED_THREAD: &[u8] = b"thread";

/// The default rate limit to initialize threads with
const DEFAULT_RATE_LIMIT: u64 = 10;

/// The maximum rate limit which may be set on thread.
const MAX_RATE_LIMIT: u64 = 32;

/// The minimum exec fee that may be set on a thread.
const MINIMUM_FEE: u64 = 1000;

/// Tracks the current state of a transaction thread on Solana.
#[account]
#[derive(Debug)]
pub struct Thread {
    /// The owner of this thread.
    pub authority: Pubkey,
    /// The cluster clock at the moment the thread was created.
    pub created_at: ClockData,
    /// The context of the thread's current execution state.
    pub exec_context: Option<ExecContext>,
    /// The number of lamports to payout to workers per execution.
    pub fee: u64,
    /// The id of the thread, given by the authority.
    pub id: String,
    /// The instructions to be executed.
    pub instructions: Vec<InstructionData>,
    /// The next instruction to be executed.
    pub next_instruction: Option<InstructionData>,
    /// Whether or not the thread is currently paused.
    pub paused: bool,
    /// The maximum number of execs allowed per slot.
    pub rate_limit: u64,
    /// The triggering event to kickoff a thread.
    pub trigger: Trigger,
}

impl Thread {
    /// Derive the pubkey of a thread account.
    pub fn pubkey(authority: Pubkey, id: String) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_THREAD, authority.as_ref(), id.as_bytes()],
            &crate::ID,
        )
        .0
    }
}

impl TryFrom<Vec<u8>> for Thread {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Thread::try_deserialize(&mut data.as_slice())
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.id.eq(&other.id)
    }
}

impl Eq for Thread {}

/// The properties of threads which are updatable.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ThreadSettings {
    pub fee: Option<u64>,
    pub instructions: Option<Vec<InstructionData>>,
    pub rate_limit: Option<u64>,
    pub trigger: Option<Trigger>,
}

/// Trait for reading and writing to a thread account.
pub trait ThreadAccount {
    /// Get the pubkey of the thread account.
    fn pubkey(&self) -> Pubkey;

    /// Initialize the account to hold thread object.
    fn init(
        &mut self,
        authority: Pubkey,
        id: String,
        instructions: Vec<InstructionData>,
        trigger: Trigger,
    ) -> Result<()>;

    /// Reallocate the memory allocation for the account.
    fn realloc(&mut self) -> Result<()>;

    fn update(&mut self, settings: ThreadSettings) -> Result<()>;
}

impl ThreadAccount for Account<'_, Thread> {
    fn pubkey(&self) -> Pubkey {
        Thread::pubkey(self.authority, self.id.clone())
    }

    fn init(
        &mut self,
        authority: Pubkey,
        id: String,
        instructions: Vec<InstructionData>,
        trigger: Trigger,
    ) -> Result<()> {
        self.authority = authority.key();
        self.created_at = Clock::get().unwrap().into();
        self.exec_context = None;
        self.fee = MINIMUM_FEE;
        self.id = id;
        self.instructions = instructions;
        self.next_instruction = None;
        self.paused = false;
        self.rate_limit = DEFAULT_RATE_LIMIT;
        self.trigger = trigger;
        Ok(())
    }

    fn realloc(&mut self) -> Result<()> {
        // Realloc memory for the thread account
        let data_len = 8 + self.try_to_vec()?.len();
        self.to_account_info().realloc(data_len, false)?;
        Ok(())
    }

    fn update(&mut self, settings: ThreadSettings) -> Result<()> {
        // If provided, update the thread's fee.
        if let Some(fee) = settings.fee {
            self.fee = fee;
        }

        // If provided, update the thread's instruction set.
        if let Some(instructions) = settings.instructions {
            self.instructions = instructions;
        }

        // If provided, update the rate limit.
        if let Some(rate_limit) = settings.rate_limit {
            require!(
                rate_limit.le(&MAX_RATE_LIMIT),
                ClockworkError::MaxRateLimitExceeded
            );
            self.rate_limit = rate_limit;
        }

        // If provided, update the thread's trigger and reset the exec context.
        if let Some(trigger) = settings.trigger {
            // Require the thread is not in the middle of processing.
            require!(self.next_instruction.is_none(), ClockworkError::ThreadBusy);
            self.trigger = trigger;
            self.exec_context = None;
        }

        Ok(())
    }
}

/// The triggering conditions of a thread.
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, PartialEq)]
pub enum Trigger {
    /// Allows a thread to be kicked off whenever the data of an account changes.
    Account {
        /// The address of the account to monitor.
        address: Pubkey,
        /// The byte offset of the account data to monitor.
        offset: usize,
        /// The size of the byte slice to monitor (must be less than 1kb)
        size: usize,
    },

    /// Allows a thread to be kicked off according to a one-time or recurring schedule.
    Cron {
        /// The schedule in cron syntax. Value must be parsable by the `clockwork_cron` package.
        schedule: String,

        /// Boolean value indicating whether triggering moments may be skipped if they are missed (e.g. due to network downtime).
        /// If false, any "missed" triggering moments will simply be executed as soon as the network comes back online.
        skippable: bool,
    },

    /// Allows a thread to be kicked off as soon as it's created.
    Immediate,
}

/// The execution context of a particular transaction thread.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecContext {
    /// Number of execs since the last tx reimbursement.
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

    /// The immediate trigger context.
    Immediate,
}
