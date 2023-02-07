use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn automation_kickoff(signatory: Pubkey, automation: Pubkey, worker: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_automation_program::ID,
        accounts: vec![
            AccountMeta::new(signatory, true),
            AccountMeta::new(automation, false),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_automation_program::instruction::AutomationKickoff {}.data(),
    }
}
