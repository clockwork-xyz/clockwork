use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use clockwork_macros::TryFromData;
use clockwork_utils::automation::{ClockData, Ix, Trigger};

pub const SEED_AUTOMATION: &[u8] = b"automation";

/// Tracks the current state of a transaction automation on Solana.
#[account]
#[derive(Debug, TryFromData)]
pub struct Automation {
    /// The owner of this automation.
    pub authority: Pubkey,
    /// The bump, used for PDA validation.
    pub bump: u8,
    /// The cluster clock at the moment the automation was created.
    pub created_at: ClockData,
    /// The context of the automation's current execution state.
    pub exec_context: Option<ExecContext>,
    /// The number of lamports to payout to workers per execution.
    pub fee: u64,
    /// The id of the automation, given by the authority.
    pub id: Vec<u8>,
    /// The instructions to be executed.
    pub instructions: Vec<Ix>,
    /// The name of the automation.
    pub name: String,
    /// The next instruction to be executed.
    pub next_instruction: Option<Ix>,
    /// Whether or not the automation is currently paused.
    pub paused: bool,
    /// The maximum number of execs allowed per slot.
    pub rate_limit: u64,
    /// The triggering event to kickoff an automation.
    pub trigger: Trigger,
}

impl Automation {
    /// Derive the pubkey of an automation account.
    pub fn pubkey(authority: Pubkey, id: Vec<u8>) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_AUTOMATION, authority.as_ref(), id.as_slice()],
            &crate::ID,
        )
        .0
    }
}

impl PartialEq for Automation {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.id.eq(&other.id)
    }
}

impl Eq for Automation {}

/// Trait for reading and writing to an automation account.
pub trait AutomationAccount {
    /// Get the pubkey of the automation account.
    fn pubkey(&self) -> Pubkey;

    /// Allocate more memory for the account.
    fn realloc(&mut self) -> Result<()>;
}

impl AutomationAccount for Account<'_, Automation> {
    fn pubkey(&self) -> Pubkey {
        Automation::pubkey(self.authority, self.id.clone())
    }

    fn realloc(&mut self) -> Result<()> {
        // Realloc memory for the automation account
        let data_len = 8 + self.try_to_vec()?.len();
        self.to_account_info().realloc(data_len, false)?;
        Ok(())
    }
}

/// The execution context of a particular transaction automation.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecContext {
    /// Index of the next instruction to be executed.
    pub exec_index: u64,

    /// Number of execs since the last tx reimbursement.
    pub execs_since_reimbursement: u64,

    /// Number of execs in this slot.
    pub execs_since_slot: u64,

    /// Slot of the last exec
    pub last_exec_at: u64,

    /// Context for the triggering condition
    pub trigger_context: TriggerContext,
}

/// The event which allowed a particular transaction automation to be triggered.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerContext {
    /// A running hash of the observed account data.
    Account {
        /// The account's data hash.
        data_hash: u64,
    },

    /// The active trigger context.
    Active,

    /// A cron execution context.
    Cron {
        /// The threshold moment the schedule was waiting for.
        started_at: i64,
    },
}

/// The properties of automations which are updatable.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AutomationSettings {
    pub fee: Option<u64>,
    pub instructions: Option<Vec<Ix>>,
    pub name: Option<String>,
    pub rate_limit: Option<u64>,
    pub trigger: Option<Trigger>,
}
