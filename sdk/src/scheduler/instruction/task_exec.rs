use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn task_exec(
    bot: Pubkey,
    config: Pubkey,
    fee: Pubkey,
    queue: Pubkey,
    task: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(bot, true),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(fee, false),
            AccountMeta::new(queue, false),
            AccountMeta::new(task, false),
        ],
        data: cronos_scheduler::instruction::TaskExec {}.data(),
    }
}
