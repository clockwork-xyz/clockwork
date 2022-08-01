use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    clockwork_scheduler::state::InstructionData as ClockworkInstructionData,
};

pub fn task_new(
    authority: Pubkey,
    ixs: Vec<Instruction>,
    payer: Pubkey,
    queue: Pubkey,
    task: Pubkey,
) -> Instruction {
    Instruction {
        program_id: clockwork_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task, false),
        ],
        data: clockwork_scheduler::instruction::TaskNew {
            ixs: ixs
                .iter()
                .map(|ix| ClockworkInstructionData::from(ix.clone()))
                .collect(),
        }
        .data(),
    }
}
