use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn queue_exec(
    task: Pubkey,
    config: Pubkey,
    delegate: Pubkey,
    fee: Pubkey,
    manager: Pubkey,
    queue: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(task, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(delegate, true),
            AccountMeta::new(fee, false),
            AccountMeta::new_readonly(manager, false),
            AccountMeta::new(queue, false),
        ],
        data: cronos_scheduler::instruction::QueueExec {}.data(),
    }
}
