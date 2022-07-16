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
    let fee_pubkey = cronos_scheduler::state::Fee::pubkey(queue);
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(fee_pubkey, false),
            AccountMeta::new(manager, false),
            AccountMeta::new(payer, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::QueueNew { schedule }.data(),
    }
}
