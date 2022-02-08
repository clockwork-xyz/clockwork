use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn task_cancel(daemon: Pubkey, task: Pubkey, owner: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(daemon, false),
            AccountMeta::new(task, false),
            AccountMeta::new(owner, true),
        ],
        data: cronos_program::instruction::TaskCancel {}.data(),
    }
}
