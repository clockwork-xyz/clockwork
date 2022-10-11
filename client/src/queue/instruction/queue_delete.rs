use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn queue_delete(authority: Pubkey, close_to: Pubkey, queue: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_queue_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(close_to, true),
            AccountMeta::new(queue, false),
        ],
        data: clockwork_queue_program::instruction::QueueDelete {}.data(),
    }
}
