use clockwork_automation_program::state::AutomationSettings;

use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn automation_update(authority: Pubkey, automation: Pubkey, settings: AutomationSettings) -> Instruction {
    Instruction {
        program_id: clockwork_automation_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(automation, false),
        ],
        data: clockwork_automation_program::instruction::AutomationUpdate { settings }.data(),
    }
}
