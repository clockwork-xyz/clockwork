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

pub fn registry_epoch_kickoff(queue: Pubkey, snapshot: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new_readonly(queue, true),
            AccountMeta::new(Registry::pubkey(), false),
            AccountMeta::new_readonly(snapshot, false),
        ],
        data: clockwork_network_program::instruction::RegistryEpochKickoff {}.data(),
    }
}
