use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn admin_queue_new(
    admin: Pubkey,
    authority: Pubkey,
    config: Pubkey,
    yogi: Pubkey,
    schedule: String,
    queue: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(yogi, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(queue, false),
        ],
        data: cronos_scheduler::instruction::AdminQueueNew { schedule }.data(),
    }
}
