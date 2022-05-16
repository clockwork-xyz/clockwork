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
    task: Pubkey,
    ixs: Vec<Instruction>,
    owner: Pubkey,
    payer: Pubkey,
    yogi: Pubkey,
    queue: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(task, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(yogi, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(queue, false),
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
