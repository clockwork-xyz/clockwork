use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn task_new(
    owner: Pubkey,
    payer: Pubkey,
    queue: Pubkey,
    schedule: String,
    task: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(payer, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task, false),
        ],
        data: cronos_scheduler::instruction::TaskNew { schedule }.data(),
    }
}
