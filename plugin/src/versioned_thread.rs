use clockwork_thread_program_v2::state::{
    ClockData, ExecContext, InstructionData, Trigger, TriggerContext,
};

#[derive(Debug)]
pub enum VersionedThread {
    V1(clockwork_thread_program_v1::state::Thread),
    V2(clockwork_thread_program_v2::state::Thread),
}

impl VersionedThread {
    pub fn paused(&self) -> bool {
        match self {
            Self::V1(t) => t.paused,
            Self::V2(t) => t.paused,
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
}
