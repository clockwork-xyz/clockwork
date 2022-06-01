use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn queue_new(
    authority: Pubkey,
    manager: Pubkey,
    payer: Pubkey,
    queue: Pubkey,
    schedule: String,
) -> Instruction {
    let fee = cronos_scheduler::state::Fee::pda(queue).0;
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(fee, false),
            AccountMeta::new(manager, false),
            AccountMeta::new(payer, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::QueueNew { schedule }.data(),
    }
}
