use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn queue_new(fee: Pubkey, owner: Pubkey, payer: Pubkey, queue: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(fee, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(payer, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::QueueNew {}.data(),
    }
}
