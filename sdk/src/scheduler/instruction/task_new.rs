use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program, sysvar,
        },
        InstructionData,
    },
    cronos_scheduler::{pda::PDA, state::InstructionData as CronosInstructionData},
};

pub fn task_new(
    task_pda: PDA,
    daemon: Pubkey,
    owner: Pubkey,
    ixs: Vec<Instruction>,
    schedule: String,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(daemon, false),
            AccountMeta::new(owner, true),
            AccountMeta::new(task_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::TaskNew {
            ixs: ixs
                .iter()
                .map(|ix| CronosInstructionData::from(ix.clone()))
                .collect(),
            schedule,
            bump: task_pda.1,
        }
        .data(),
    }
}
