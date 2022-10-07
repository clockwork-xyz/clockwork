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

pub fn queue_create(
    authority: Pubkey,
    id: String,
    kickoff_instruction: Instruction,
    payer: Pubkey,
    queue: Pubkey,
    trigger: Trigger,
) -> Instruction {
    Instruction {
        program_id: clockwork_queue_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_queue_program::instruction::QueueCreate {
            id,
            kickoff_instruction: ClockworkInstructionData::from(kickoff_instruction),
            trigger,
        }
        .data(),
    }
}
