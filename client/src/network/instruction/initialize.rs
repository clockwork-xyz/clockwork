use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    clockwork_network_program::objects::*,
};

pub fn initialize(admin: Pubkey, mint: Pubkey) -> Instruction {
    let epoch_pubkey = Epoch::pubkey(0);
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(Config::pubkey(), false),
            AccountMeta::new(epoch_pubkey, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(Registry::pubkey(), false),
            AccountMeta::new(Rotator::pubkey(), false),
            AccountMeta::new(Snapshot::pubkey(epoch_pubkey), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network_program::instruction::Initialize {}.data(),
    }
}
