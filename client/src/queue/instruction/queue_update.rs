use clockwork_queue_program::objects::QueueSettings;

use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn queue_update(authority: Pubkey, queue: Pubkey, settings: QueueSettings) -> Instruction {
    Instruction {
        program_id: clockwork_queue_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_queue_program::instruction::QueueUpdate { settings }.data(),
    }
}
