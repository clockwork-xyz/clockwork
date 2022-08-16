use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey, mint: Pubkey) -> Instruction {
    let authority_pubkey = clockwork_network::state::Authority::pubkey();
    let config_pubkey = clockwork_network::state::Config::pubkey();
    let rotator_pubkey = clockwork_network::state::Rotator::pubkey();
    let registry_pubkey = clockwork_network::state::Registry::pubkey();
    let snapshot_pubkey = clockwork_network::state::Snapshot::pubkey(0);
    let snapshot_queue = clockwork_crank::state::Queue::pubkey(authority_pubkey, "snapshot".into());

    Instruction {
        program_id: clockwork_network::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(authority_pubkey, false),
            AccountMeta::new_readonly(clockwork_crank::ID, false),
            AccountMeta::new(config_pubkey, false),
            AccountMeta::new(rotator_pubkey, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(registry_pubkey, false),
            AccountMeta::new(snapshot_pubkey, false),
            AccountMeta::new(snapshot_queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network::instruction::Initialize {}.data(),
    }
}
