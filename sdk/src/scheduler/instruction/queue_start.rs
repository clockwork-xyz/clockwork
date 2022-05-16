use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn queue_start(delegate: Pubkey, manager: Pubkey, queue: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(delegate, true),
            AccountMeta::new(manager, false),
            AccountMeta::new(queue, false),
        ],
        data: cronos_scheduler::instruction::QueueStart {}.data(),
    }
}
