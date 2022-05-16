use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn queue_new(
    owner: Pubkey,
    payer: Pubkey,
    manager: Pubkey,
    schedule: String,
    queue: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(payer, true),
            AccountMeta::new(manager, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(queue, false),
        ],
        data: cronos_scheduler::instruction::QueueNew { schedule }.data(),
    }
}
