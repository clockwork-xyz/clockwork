use anchor_lang::{
    prelude::{Error, Pubkey},
    AccountDeserialize,
};
use clockwork_automation_program_v1::state::Thread as AutomationV1;
use clockwork_automation_program_v2::state::{
    Automation as AutomationV2, ClockData, ExecContext, InstructionData, Trigger, TriggerContext,
};

#[derive(Debug)]
pub enum VersionedAutomation {
    V1(AutomationV1),
    V2(AutomationV2),
}

impl VersionedAutomation {
    pub fn authority(&self) -> Pubkey {
        dbg!("versioned authority()", self);
        match self {
            Self::V1(t) => t.authority,
            Self::V2(t) => t.authority,
        }
    }

    pub fn created_at(&self) -> ClockData {
        dbg!("versioned created_at()", self);
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
        dbg!("versioned exec_context()", self);
        match self {
            Self::V1(t) => t.exec_context.map(|e| ExecContext {
                exec_index: 0,
                execs_since_reimbursement: e.execs_since_reimbursement,
                execs_since_slot: e.execs_since_slot,
                last_exec_at: e.last_exec_at,
                trigger_context: unsafe {
                    std::mem::transmute::<
                        clockwork_automation_program_v1::state::TriggerContext,
                        TriggerContext,
                    >(e.trigger_context)
                },
            }),
            Self::V2(t) => t.exec_context,
        }
    }

    pub fn next_instruction(&self) -> Option<InstructionData> {
        dbg!("versioned next_instruction()", self);
        match self {
            Self::V1(t) => unsafe {
                std::mem::transmute_copy::<
                    Option<clockwork_automation_program_v1::state::InstructionData>,
                    Option<InstructionData>,
                >(&t.next_instruction)
            },
            Self::V2(t) => t.next_instruction.clone(),
        }
    }

    pub fn paused(&self) -> bool {
        dbg!("versioned paused()", self);
        match self {
            Self::V1(t) => t.paused,
            Self::V2(t) => t.paused,
        }
    }

    pub fn rate_limit(&self) -> u64 {
        dbg!("versioned rate_limit()", self);
        match self {
            Self::V1(t) => t.rate_limit,
            Self::V2(t) => t.rate_limit,
        }
    }

    pub fn trigger(&self) -> Trigger {
        dbg!("versioned trigger()", self);
        match self {
            Self::V1(t) => unsafe {
                // TODO Handle case where we rename trigger value
                std::mem::transmute_copy::<clockwork_automation_program_v1::state::Trigger, Trigger>(
                    &t.trigger,
                )
            },
            Self::V2(t) => t.trigger.clone(),
        }
    }
}

impl AccountDeserialize for VersionedAutomation {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Self::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        // Try first to deserialize into AutomationV2.
        // If this fails, try to deserialize into AutomationV1.
        dbg!("versioned try_deserialized_unchecked()");
        match AutomationV2::try_deserialize(buf) {
            Err(_err) => Ok(VersionedAutomation::V1(AutomationV1::try_deserialize(buf)?)),
            Ok(t) => Ok(VersionedAutomation::V2(t)),
        }
    }
}

impl TryFrom<Vec<u8>> for VersionedAutomation {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        VersionedAutomation::try_deserialize(&mut data.as_slice())
    }
}
