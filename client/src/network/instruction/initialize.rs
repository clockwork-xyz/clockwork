use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey, mint: Pubkey) -> Instruction {
    let authority = cronos_network::state::Authority::pda().0;
    let config = cronos_network::state::Config::pda().0;
    let rotator = cronos_network::state::Rotator::pda().0;
    let registry = cronos_network::state::Registry::pda().0;
    let snapshot = cronos_network::state::Snapshot::pda(0).0;

    let manager = cronos_scheduler::state::Manager::pubkey(authority);
    let snapshot_queue = cronos_scheduler::state::Queue::pubkey(manager, 0);
    let snapshot_fee = cronos_scheduler::state::Fee::pubkey(snapshot_queue);
    let snapshot_task = cronos_scheduler::state::Task::pubkey(snapshot_queue, 0);

    Instruction {
        program_id: cronos_network::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(authority, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(config, false),
            AccountMeta::new(rotator, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(registry, false),
            AccountMeta::new_readonly(cronos_scheduler::ID, false),
            AccountMeta::new(snapshot, false),
            AccountMeta::new_readonly(system_program::ID, false),
            // Additional accounts
            // AccountMeta::new(rotator_fee, false),
            // AccountMeta::new(rotator_queue, false),
            AccountMeta::new(manager, false),
            AccountMeta::new(snapshot_fee, false),
            AccountMeta::new(snapshot_queue, false),
            AccountMeta::new(snapshot_task, false),
        ],
        data: cronos_network::instruction::Initialize {}.data(),
    }
}
