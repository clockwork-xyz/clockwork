use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};
use cronos_program::state::InstructionData as CronosInstructionData;
use cronos_program::{pda::PDA, state::Task};

pub fn task_create(
    task_pda: PDA,
    daemon: Pubkey,
    owner: Pubkey,
    ix: Instruction,
    exec_at: i64,
    stop_at: i64,
    recurr: i64,
) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(daemon, false),
            AccountMeta::new(owner, true),
            AccountMeta::new(task_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_program::instruction::TaskCreate {
            ix: CronosInstructionData::from(ix),
            exec_at,
            stop_at,
            recurr,
            bump: task_pda.1,
        }
        .data(),
    }
}
