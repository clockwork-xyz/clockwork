use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn task_execute(
    config: Pubkey,
    daemon: Pubkey,
    fee: Pubkey,
    task: Pubkey,
    worker: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(daemon, false),
            AccountMeta::new(fee, false),
            AccountMeta::new(task, false),
            AccountMeta::new(worker, true),
        ],
        data: cronos_program::instruction::TaskExecute {}.data(),
    }
}
