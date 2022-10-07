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
    // let config_pubkey = clockwork_network_program::objects::Config::pubkey();
    // let rotator_pubkey = clockwork_network_program::objects::Rotator::pubkey();
    // let registry_pubkey = clockwork_network_program::objects::Registry::pubkey();
    // let snapshot_pubkey = clockwork_network_program::objects::Snapshot::pubkey(0);
    // let snapshot_queue =
    //     clockwork_queue_program::objects::Queue::pubkey(authority_pubkey, "snapshot".into());

    let epoch_pubkey = Epoch::pubkey(0);
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(Config::pubkey(), false),
            AccountMeta::new(epoch_pubkey, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(Rotator::pubkey(), false),
            AccountMeta::new(Registry::pubkey(), false),
            AccountMeta::new(Snapshot::pubkey(epoch_pubkey), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network_program::instruction::Initialize {}.data(),
    }
}
