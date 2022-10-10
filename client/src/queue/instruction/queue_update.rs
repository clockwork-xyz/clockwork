use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    clockwork_queue_program::objects::Trigger,
    clockwork_utils::InstructionData as ClockworkInstructionData,
};

pub fn queue_update(
    authority: Pubkey,
    queue: Pubkey,
    kickoff_instruction: Option<ClockworkInstructionData>,
    rate_limit: Option<u64>,
    trigger: Option<Trigger>,
) -> Instruction {
    Instruction {
        program_id: clockwork_queue_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_queue_program::instruction::QueueUpdate {
            kickoff_instruction,
            rate_limit,
            trigger,
        }
        .data(),
    }
}
