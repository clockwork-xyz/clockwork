use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    cronos_scheduler::state::InstructionData as CronosInstructionData,
};

pub fn queue_sign(queue: Pubkey, owner: Pubkey, instruction: Instruction) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(owner, true),
            AccountMeta::new(queue, false),
        ],
        data: cronos_scheduler::instruction::QueueSign {
            ix: CronosInstructionData::from(instruction),
        }
        .data(),
    }
}
