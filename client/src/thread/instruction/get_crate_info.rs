use anchor_lang::{solana_program::instruction::Instruction, InstructionData};

pub fn get_crate_info() -> Instruction {
    Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![],
        data: clockwork_thread_program::instruction::GetCrateInfo {}.data(),
    }
}
