use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    cronos_scheduler::state::InstructionData as CronosInstructionData,
};

pub fn task_new(
    authority: Pubkey,
    ixs: Vec<Instruction>,
    manager: Pubkey,
    payer: Pubkey,
    queue: Pubkey,
    task: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(manager, false),
            AccountMeta::new(payer, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task, false),
        ],
        data: cronos_scheduler::instruction::TaskNew {
            ixs: ixs
                .iter()
                .map(|ix| CronosInstructionData::from(ix.clone()))
                .collect(),
        }
        .data(),
    }
}
