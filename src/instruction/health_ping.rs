use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn health_ping(
    authority: Pubkey,
    config: Pubkey,
    daemon: Pubkey,
    health: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(daemon, true),
            AccountMeta::new(health, false),
        ],
        data: cronos_program::instruction::HealthPing {}.data(),
    }
}
