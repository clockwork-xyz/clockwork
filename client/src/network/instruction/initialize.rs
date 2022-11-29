use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    clockwork_network_program::state::*,
};

pub fn initialize(admin: Pubkey, mint: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(Config::pubkey(), false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(Registry::pubkey(), false),
            AccountMeta::new(Snapshot::pubkey(0), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network_program::instruction::Initialize {}.data(),
    }
}
