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

pub fn task_new(owner: Pubkey, queue: Pubkey, schedule: String, task_pda: PDA) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(owner, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task_pda.0, false),
        ],
        data: cronos_scheduler::instruction::TaskNew {
            bump: task_pda.1,
            schedule,
        }
        .data(),
    }
}

// ixs
// .iter()
// .map(|ix| CronosInstructionData::from(ix.clone()))
// .collect(),
