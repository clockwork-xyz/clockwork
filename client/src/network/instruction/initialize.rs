use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey, mint: Pubkey) -> Instruction {
    let authority_pubkey = cronos_network::state::Authority::pubkey();
    let config_pubkey = cronos_network::state::Config::pubkey();
    let rotator_pubkey = cronos_network::state::Rotator::pubkey();
    let registry_pubkey = cronos_network::state::Registry::pubkey();
    let snapshot_pubkey = cronos_network::state::Snapshot::pubkey(0);
    let snapshot_queue = cronos_scheduler::state::Queue::pubkey(authority_pubkey, 0);
    let snapshot_task = cronos_scheduler::state::Task::pubkey(snapshot_queue, 0);

    Instruction {
        program_id: cronos_network::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(authority_pubkey, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(config_pubkey, false),
            AccountMeta::new(rotator_pubkey, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(registry_pubkey, false),
            AccountMeta::new_readonly(cronos_scheduler::ID, false),
            AccountMeta::new(snapshot_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
            // Additional accounts
            AccountMeta::new(snapshot_queue, false),
            AccountMeta::new(snapshot_task, false),
        ],
        data: cronos_network::instruction::Initialize {}.data(),
    }
}
