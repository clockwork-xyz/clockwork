use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn task_exec(delegate: Pubkey, manager: Pubkey, queue: Pubkey, task: Pubkey) -> Instruction {
    let config_pubkey = cronos_scheduler::state::Config::pubkey();
    let fee_pubkey = cronos_scheduler::state::Fee::pubkey(queue);
    let pool = cronos_pool::state::Pool::pda().0;
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config_pubkey, false),
            AccountMeta::new(delegate, true),
            AccountMeta::new(fee_pubkey, false),
            AccountMeta::new_readonly(manager, false),
            AccountMeta::new_readonly(pool, false),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task, false),
        ],
        data: cronos_scheduler::instruction::TaskExec {}.data(),
    }
}
