use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn task_exec(delegate: Pubkey, manager: Pubkey, queue: Pubkey, task: Pubkey) -> Instruction {
    let config = cronos_scheduler::state::Config::pda().0;
    let fee = cronos_scheduler::state::Fee::pda(queue).0;
    let pool = cronos_pool::state::Pool::pda().0;
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(delegate, true),
            AccountMeta::new(fee, false),
            AccountMeta::new_readonly(manager, false),
            AccountMeta::new_readonly(pool, false),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task, false),
        ],
        data: cronos_scheduler::instruction::TaskExec {}.data(),
    }
}
