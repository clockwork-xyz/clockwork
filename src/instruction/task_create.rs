use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};
use cronos_program::pda::PDA;
use cronos_program::state::InstructionData as CronosInstructionData;

pub fn task_create(
    task_pda: PDA,
    daemon: Pubkey,
    owner: Pubkey,
    instruction: Instruction,
    exec_at: i64,
    stop_at: i64,
    recurr: i64,
) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(daemon, false),
            AccountMeta::new(task_pda.0, false),
            AccountMeta::new(owner, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_program::instruction::TaskCreate {
            instruction_data: CronosInstructionData::from(instruction),
            exec_at,
            stop_at,
            recurr,
            bump: task_pda.1,
        }
        .data(),
    }
}
