use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    clockwork_network_program::state::*,
};

pub fn registry_epoch_kickoff(snapshot: Pubkey, thread: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new(Registry::pubkey(), false),
            AccountMeta::new_readonly(snapshot, false),
            AccountMeta::new_readonly(thread, true),
        ],
        data: clockwork_network_program::instruction::RegistryEpochKickoff {}.data(),
    }
}
