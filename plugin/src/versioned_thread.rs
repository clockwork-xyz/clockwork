use anchor_lang::{
    prelude::{Error, Pubkey},
    AccountDeserialize,
};
use clockwork_thread_program_v2::state::{
    ClockData, ExecContext, InstructionData, Trigger, TriggerContext,
};

#[derive(Debug)]
pub enum VersionedThread {
    V1(clockwork_thread_program_v1::state::Thread),
    V2(clockwork_thread_program_v2::state::Thread),
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
            Self::V1(t) => unsafe {
                std::mem::transmute_copy::<clockwork_thread_program_v1::state::ClockData, ClockData>(
                    &t.created_at,
                )
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
                    std::mem::transmute::<
                        clockwork_thread_program_v1::state::TriggerContext,
                        TriggerContext,
                    >(e.trigger_context)
                },
            }),
            Self::V2(t) => t.exec_context,
        }
    }

    pub fn next_instruction(&self) -> Option<InstructionData> {
        match self {
            Self::V1(t) => unsafe {
                std::mem::transmute_copy::<
                    Option<clockwork_thread_program_v1::state::InstructionData>,
                    Option<InstructionData>,
                >(&t.next_instruction)
            },
            Self::V2(t) => t.next_instruction.clone(),
        }
    }

    pub fn paused(&self) -> bool {
        match self {
            Self::V1(t) => t.paused,
            Self::V2(t) => t.paused,
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
            Self::V1(t) => unsafe {
                std::mem::transmute_copy::<clockwork_thread_program_v1::state::Trigger, Trigger>(
                    &t.trigger,
                )
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
        // Try first to deserialize into thread v2.
        // If this fails, try to deserialize into thread v1.
        match clockwork_thread_program_v2::state::Thread::try_deserialize(buf) {
            Err(_err) => Ok(VersionedThread::V1(
                clockwork_thread_program_v1::state::Thread::try_deserialize(buf)?,
            )),
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
