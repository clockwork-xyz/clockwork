use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    clockwork_thread_program::objects::Trigger,
    clockwork_utils::InstructionData as ClockworkInstructionData,
};

pub fn thread_create(
    authority: Pubkey,
    id: String,
    kickoff_instruction: ClockworkInstructionData,
    payer: Pubkey,
    thread: Pubkey,
    trigger: Trigger,
) -> Instruction {
    Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new(thread, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_thread_program::instruction::ThreadCreate {
            id,
            kickoff_instruction,
            trigger,
        }
        .data(),
    }
}
