use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(
    admin: Pubkey,
    authority: Pubkey,
    config: Pubkey,
    fee: Pubkey,
    pool_pubkey: Pubkey,
    queue: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(authority, false),
            AccountMeta::new(config, false),
            AccountMeta::new(fee, false),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::Initialize { pool_pubkey }.data(),
    }
}
