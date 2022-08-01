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
    let snapshot_queue =
        clockwork_scheduler::state::Queue::pubkey(authority_pubkey, "snapshot".into());
    let snapshot_task = clockwork_scheduler::state::Task::pubkey(snapshot_queue, 0);
    let cleanup_queue =
        clockwork_scheduler::state::Queue::pubkey(authority_pubkey, "cleanup".into());
    let cleanup_task = clockwork_scheduler::state::Task::pubkey(cleanup_queue, 0);

    Instruction {
        program_id: clockwork_network::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(authority_pubkey, false),
            AccountMeta::new(cleanup_queue, false),
            AccountMeta::new(cleanup_task, false),
            AccountMeta::new(config_pubkey, false),
            AccountMeta::new(rotator_pubkey, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(registry_pubkey, false),
            AccountMeta::new_readonly(clockwork_scheduler::ID, false),
            AccountMeta::new(snapshot_pubkey, false),
            AccountMeta::new(snapshot_queue, false),
            AccountMeta::new(snapshot_task, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network::instruction::Initialize {}.data(),
    }
}
