use anchor_lang::{
    prelude::{Error, Pubkey},
    AccountDeserialize,
};
use clockwork_thread_program_v1::state::Thread as ThreadV1;
use clockwork_thread_program_v2::state::{
    SerializableAccount, Thread as ThreadV2, ClockData, ExecContext, SerializableInstruction, Trigger,
    TriggerContext,
};

#[derive(Clone, Debug)]
pub enum VersionedThread {
    V1(ThreadV1),
    V2(ThreadV2),
}

impl VersionedThread {
    pub fn authority(&self) -> Pubkey {
        dbg!("versioned authority()");
        match self {
            Self::V1(t) => t.authority,
            Self::V2(t) => t.authority,
        }
    }

    pub fn created_at(&self) -> ClockData {
        dbg!("versioned created_at()");
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
        dbg!("versioned exec_context()");
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

    pub fn next_instruction(&self) -> Option<SerializableInstruction> {
        dbg!("versioned next_instruction()");
        match self {
            Self::V1(t) => match &t.next_instruction {
                None => None,
                Some(ix) => Some(SerializableInstruction {
                    program_id: ix.program_id,
                    accounts: ix
                        .accounts
                        .iter()
                        .map(|a| unsafe {
                            std::mem::transmute_copy::<
                                clockwork_thread_program_v1::state::AccountMetaData,
                                SerializableAccount,
                            >(a)
                        })
                        .collect::<Vec<SerializableAccount>>(),
                    data: ix.data.clone(),
                }),
            },
            Self::V2(t) => t.next_instruction.clone(),
        }
    }

    pub fn paused(&self) -> bool {
        dbg!("versioned paused()");
        match self {
            Self::V1(t) => t.paused,
            Self::V2(t) => t.paused,
        }
    }

    pub fn rate_limit(&self) -> u64 {
        dbg!("versioned rate_limit()");
        match self {
            Self::V1(t) => t.rate_limit,
            Self::V2(t) => t.rate_limit,
        }
    }

    pub fn trigger(&self) -> Trigger {
        dbg!("versioned trigger()");
        match self {
            Self::V1(t) => unsafe {
                // TODO Handle case where we rename trigger value
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
        // Try first to deserialize into ThreadV2.
        // If this fails, try to deserialize into ThreadV1.
        dbg!("versioned try_deserialized_unchecked()");
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
