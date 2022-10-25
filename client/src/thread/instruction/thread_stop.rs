use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn thread_stop(authority: Pubkey, thread: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(thread, false),
        ],
        data: clockwork_thread_program::instruction::ThreadStop {}.data(),
    }
}
