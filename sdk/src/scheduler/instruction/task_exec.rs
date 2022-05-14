use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn task_exec(
    action: Pubkey,
    config: Pubkey,
    delegate: Pubkey,
    fee: Pubkey,
    queue: Pubkey,
    task: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(action, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(delegate, true),
            AccountMeta::new(fee, false),
            AccountMeta::new_readonly(queue, false),
            AccountMeta::new(task, false),
        ],
        data: cronos_scheduler::instruction::TaskExec {}.data(),
    }
}
