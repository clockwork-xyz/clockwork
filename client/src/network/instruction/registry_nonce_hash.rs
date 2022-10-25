use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    clockwork_network_program::objects::*,
};

pub fn registry_nonce_hash(thread: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new_readonly(thread, true),
            AccountMeta::new(Registry::pubkey(), false),
        ],
        data: clockwork_network_program::instruction::RegistryNonceHash {}.data(),
    }
}
