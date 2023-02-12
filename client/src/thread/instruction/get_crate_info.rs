use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        system_program,
    },
    InstructionData,
};

pub fn get_crate_info() -> Instruction {
    Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![AccountMeta::new_readonly(system_program::ID, false)],
        data: clockwork_thread_program::instruction::GetCrateInfo {}.data(),
    }
}
