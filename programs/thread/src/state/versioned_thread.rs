use anchor_lang::{
    prelude::{borsh::BorshSchema, *},
    AccountDeserialize,
};

use crate::{
    ClockData, ExecContext, SerializableInstruction, Thread as ThreadV2, Trigger, TriggerContext,
};

use super::SEED_THREAD;

#[derive(Clone, Debug, PartialEq)]
pub enum VersionedThread {
    V1(ThreadV1),
    V2(ThreadV2),
}

impl VersionedThread {
    pub fn authority(&self) -> Pubkey {
        match self {
            Self::V1(t) => t.authority,
            Self::V2(t) => t.authority,
        }
    }

    pub fn created_at(&self) -> ClockData {
        match self {
            Self::V1(t) => ClockData {
                slot: t.created_at.slot,
                epoch: t.created_at.epoch,
                unix_timestamp: t.created_at.unix_timestamp,
            },
            Self::V2(t) => t.created_at.clone(),
        }
    }

    pub fn exec_context(&self) -> Option<ExecContext> {
        match self {
            Self::V1(t) => t.exec_context.map(|e| ExecContext {
                exec_index: 0,
                execs_since_reimbursement: e.execs_since_reimbursement,
                execs_since_slot: e.execs_since_slot,
                last_exec_at: e.last_exec_at,
                trigger_context: unsafe {
                    std::mem::transmute::<TriggerContextV1, TriggerContext>(e.trigger_context)
                },
            }),
            Self::V2(t) => t.exec_context,
        }
    }

    pub fn id(&self) -> Vec<u8> {
        match self {
            Self::V1(t) => t.id.as_bytes().to_vec(),
            Self::V2(t) => t.id.clone(),
        }
    }

    pub fn next_instruction(&self) -> Option<SerializableInstruction> {
        match self {
            Self::V1(t) => t.next_instruction.clone(),
            Self::V2(t) => t.next_instruction.clone(),
        }
    }

    pub fn paused(&self) -> bool {
        match self {
            Self::V1(t) => t.paused,
            Self::V2(t) => t.paused,
        }
    }

    pub fn program_id(&self) -> Pubkey {
        match self {
            Self::V1(_) => clockwork_thread_program_v1::ID,
            Self::V2(_) => crate::ID,
        }
    }

    pub fn pubkey(&self) -> Pubkey {
        match self {
            Self::V1(_) => {
                ThreadV1::pubkey(self.authority(), String::from_utf8(self.id()).unwrap())
            }
            Self::V2(_) => ThreadV2::pubkey(self.authority(), self.id()),
        }
    }

    pub fn rate_limit(&self) -> u64 {
        match self {
            Self::V1(t) => t.rate_limit,
            Self::V2(t) => t.rate_limit,
        }
    }

    pub fn trigger(&self) -> Trigger {
        match self {
            Self::V1(t) => match &t.trigger {
                TriggerV1::Account {
                    address,
                    offset,
                    size,
                } => Trigger::Account {
                    address: *address,
                    offset: *offset as u64,
                    size: *size as u64,
                },
                TriggerV1::Cron {
                    schedule,
                    skippable,
                } => Trigger::Cron {
                    schedule: schedule.clone(),
                    skippable: *skippable,
                },
                TriggerV1::Immediate => Trigger::Now,
            },
            Self::V2(t) => t.trigger.clone(),
        }
    }
}

impl AccountDeserialize for VersionedThread {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Self::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        // Try first to deserialize into ThreadV2.
        // If this fails, try to deserialize into ThreadV1.
        match ThreadV2::try_deserialize(buf) {
            Err(_err) => Ok(VersionedThread::V1(ThreadV1::try_deserialize(buf)?)),
            Ok(t) => Ok(VersionedThread::V2(t)),
        }
    }
}

impl TryFrom<Vec<u8>> for VersionedThread {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        VersionedThread::try_deserialize(&mut data.as_slice())
    }
}

// V1
pub mod clockwork_thread_program_v1 {
    use anchor_lang::declare_id;

    declare_id!("3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv");
}

/// Tracks the current state of a transaction thread on Solana.
#[account]
#[derive(Debug)]
pub struct ThreadV1 {
    /// The owner of this thread.
    pub authority: Pubkey,
    /// The cluster clock at the moment the thread was created.
    pub created_at: ClockDataV1,
    /// The context of the thread's current execution state.
    pub exec_context: Option<ExecContextV1>,
    /// The number of lamports to payout to workers per execution.
    pub fee: u64,
    /// The id of the thread, given by the authority.
    pub id: String,
    /// The instruction to kick-off the thread.
    pub kickoff_instruction: SerializableInstruction,
    /// The next instruction in the thread.
    pub next_instruction: Option<SerializableInstruction>,
    /// Whether or not the thread is currently paused.
    pub paused: bool,
    /// The maximum number of execs allowed per slot.
    pub rate_limit: u64,
    /// The triggering event to kickoff a thread.
    pub trigger: TriggerV1,
}

impl ThreadV1 {
    /// Derive the pubkey of a thread account.
    pub fn pubkey(authority: Pubkey, id: String) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_THREAD, authority.as_ref(), id.as_bytes()],
            &clockwork_thread_program_v1::ID,
        )
        .0
    }
}

impl PartialEq for ThreadV1 {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.id.eq(&other.id)
    }
}

/// The triggering conditions of a thread.
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, PartialEq)]
pub enum TriggerV1 {
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
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ExecContextV1 {
    /// Number of execs since the last tx reimbursement.
    pub execs_since_reimbursement: u64,

    /// Number of execs in this slot.
    pub execs_since_slot: u64,

    /// Slot of the last exec
    pub last_exec_at: u64,

    /// Context for the triggering condition
    pub trigger_context: TriggerContextV1,
}

/// The event which allowed a particular transaction thread to be triggered.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TriggerContextV1 {
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

/// The clock object, representing a specific moment in time recorded by a Solana cluster.
#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct ClockDataV1 {
    /// The current slot.
    pub slot: u64,
    /// The timestamp of the first slot in this Solana epoch.
    pub epoch_start_timestamp: i64,
    /// The bank epoch.
    pub epoch: u64,
    /// The future epoch for which the leader schedule has most recently been calculated.
    pub leader_schedule_epoch: u64,
    /// Originally computed from genesis creation time and network time
    /// in slots (drifty); corrected using validator timestamp oracle as of
    /// timestamp_correction and timestamp_bounding features.
    pub unix_timestamp: i64,
}
