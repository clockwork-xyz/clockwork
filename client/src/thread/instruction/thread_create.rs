use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    clockwork_thread_program::state::{InstructionData as ClockworkInstructionData, Trigger},
};

pub fn thread_create(
    authority: Pubkey,
    id: Vec<u8>,
    instructions: Vec<ClockworkInstructionData>,
    payer: Pubkey,
    thread: Pubkey,
    trigger: Trigger,
) -> Instruction {
    Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(thread, false),
        ],
        data: clockwork_thread_program::instruction::ThreadCreate {
            id,
            instructions,
            trigger,
        }
        .data(),
    }
}
