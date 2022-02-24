use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn admin_reset_health(admin: Pubkey, config: Pubkey, health: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(health, false),
        ],
        data: cronos_program::instruction::AdminResetHealth {}.data(),
    }
}
