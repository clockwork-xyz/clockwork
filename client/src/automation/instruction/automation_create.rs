use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    clockwork_automation_program::state::{InstructionData as ClockworkInstructionData, Trigger},
};

pub fn automation_create(
    amount: u64,
    authority: Pubkey,
    id: Vec<u8>,
    instructions: Vec<ClockworkInstructionData>,
    payer: Pubkey,
    automation: Pubkey,
    trigger: Trigger,
) -> Instruction {
    Instruction {
        program_id: clockwork_automation_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(automation, false),
        ],
        data: clockwork_automation_program::instruction::AutomationCreate {
            amount,
            id,
            instructions,
            trigger,
        }
        .data(),
    }
}
