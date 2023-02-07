use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn automation_reset(authority: Pubkey, automation: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_automation_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(automation, false),
        ],
        data: clockwork_automation_program::instruction::AutomationReset {}.data(),
    }
}
