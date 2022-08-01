use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn queue_new(
    authority: Pubkey,
    balance: u64,
    name: String,
    payer: Pubkey,
    queue: Pubkey,
    schedule: String,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::QueueNew {
            balance,
            name,
            schedule,
        }
        .data(),
    }
}
