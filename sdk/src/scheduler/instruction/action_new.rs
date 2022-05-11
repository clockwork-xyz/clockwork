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

pub fn action_new(
    action: Pubkey,
    ixs: Vec<Instruction>,
    owner: Pubkey,
    payer: Pubkey,
    queue: Pubkey,
    task: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(action, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task, false),
        ],
        data: cronos_scheduler::instruction::ActionNew {
            ixs: ixs
                .iter()
                .map(|ix| CronosInstructionData::from(ix.clone()))
                .collect(),
        }
        .data(),
    }
}
