use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program, sysvar,
        },
        InstructionData,
    },
    cronos_scheduler::pda::PDA,
};

pub fn admin_task_new(
    task_pda: PDA,
    admin: Pubkey,
    authority: Pubkey,
    config: Pubkey,
    queue: Pubkey,
    schedule: String,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task_pda.0, false),
        ],
        data: cronos_scheduler::instruction::AdminTaskNew {
            schedule,
            bump: task_pda.1,
        }
        .data(),
    }
}
