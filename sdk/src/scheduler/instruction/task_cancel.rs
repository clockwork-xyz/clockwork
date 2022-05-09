use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn task_cancel(queue: Pubkey, task: Pubkey, owner: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(owner, true),
            AccountMeta::new_readonly(queue, false),
            AccountMeta::new(task, false),
        ],
        data: cronos_scheduler::instruction::TaskCancel {}.data(),
    }
}
