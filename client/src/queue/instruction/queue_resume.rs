use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn queue_resume(authority: Pubkey, queue: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_queue_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(queue, false),
        ],
        data: clockwork_queue_program::instruction::QueueResume {}.data(),
    }
}
