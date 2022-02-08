use crate::pda::PDA;
use anchor_client::anchor_lang::InstructionData;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

pub fn admin_schedule_health_check(
    task_pda: PDA,
    admin: Pubkey,
    authority: Pubkey,
    config: Pubkey,
    daemon: Pubkey,
    health: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new_readonly(daemon, false),
            AccountMeta::new(health, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task_pda.0, false),
        ],
        data: cronos_program::instruction::AdminScheduleHealthCheck { bump: task_pda.1 }.data(),
    }
}
