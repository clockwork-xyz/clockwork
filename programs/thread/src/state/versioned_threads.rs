use anchor_lang::{
    prelude::{Error, Pubkey},
    AccountDeserialize,
};
use clockwork_thread_program_v1::state::Thread as ThreadV1;

use crate::{
    ClockData, ExecContext, SerializableAccount, SerializableInstruction, Thread as ThreadV2,
    Trigger, TriggerContext,
};

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
                    std::mem::transmute::<
                        clockwork_thread_program_v1::state::TriggerContext,
                        TriggerContext,
                    >(e.trigger_context)
                },
            }),
            Self::V2(t) => t.exec_context,
        }
    }

    pub fn id(&self) -> VersionedID {
        match self {
            Self::V1(t) => VersionedID::String(t.id.clone()),
            Self::V2(t) => VersionedID::Vec(t.id.clone())
        }
    }

    pub fn next_instruction(&self) -> Option<SerializableInstruction> {
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
                ThreadV1::pubkey(self.authority(), self.id().to_string().unwrap())
            },
            Self::V2(_) => {
                ThreadV2::pubkey(self.authority(), self.id().to_vec().unwrap())
            }
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
                clockwork_thread_program_v1::state::Trigger::Account {
                    address,
                    offset,
                    size,
                } => Trigger::Account {
                    address: *address,
                    offset: *offset as u64,
                    size: *size as u64,
                },
                clockwork_thread_program_v1::state::Trigger::Cron {
                    schedule,
                    skippable,
                } => Trigger::Cron {
                    schedule: schedule.clone(),
                    skippable: *skippable,
                },
                clockwork_thread_program_v1::state::Trigger::Immediate => Trigger::Now,
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

#[derive(Clone, Debug)]
pub enum VersionedID {
    String(String),
    Vec(Vec<u8>),
}

impl VersionedID {
    pub fn to_string(self) -> Option<String> {
        if let VersionedID::String(id) = self {
            return Some(id);
        } 
        None
    }

    pub fn to_vec(self) -> Option<Vec<u8>> {
        if let VersionedID::Vec(id) = self {
            return Some(id);
        }
        None
    }
}