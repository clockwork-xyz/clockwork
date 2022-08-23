use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    clockwork_crank::state::{InstructionData as ClockworkInstructionData, Trigger},
};

pub fn queue_create(
    authority: Pubkey,
    instruction: Instruction,
    name: String,
    payer: Pubkey,
    queue: Pubkey,
    trigger: Trigger,
) -> Instruction {
    Instruction {
        program_id: clockwork_crank::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_crank::instruction::QueueCreate {
            instruction: ClockworkInstructionData::from(instruction),
            name,
            trigger,
        }
        .data(),
    }
}
