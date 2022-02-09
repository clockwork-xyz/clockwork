use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn admin_cancel_task(
    admin: Pubkey,
    authority: Pubkey,
    config: Pubkey,
    daemon: Pubkey,
    task: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new_readonly(daemon, false),
            AccountMeta::new(task, false),
        ],
        data: cronos_program::instruction::AdminCancelTask {}.data(),
    }
}
