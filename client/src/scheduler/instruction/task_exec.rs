use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn task_exec(queue: Pubkey, task: Pubkey, worker: Pubkey) -> Instruction {
    let config_pubkey = clockwork_scheduler::state::Config::pubkey();
    let fee_pubkey = clockwork_scheduler::state::Fee::pubkey(worker);
    let pool_pubkey = clockwork_pool::state::Pool::pubkey();
    Instruction {
        program_id: clockwork_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(config_pubkey, false),
            AccountMeta::new(fee_pubkey, false),
            AccountMeta::new_readonly(pool_pubkey, false),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task, false),
            AccountMeta::new(worker, true),
        ],
        data: clockwork_scheduler::instruction::TaskExec {}.data(),
    }
}
