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
    let cycler = cronos_network::state::Cycler::pda().0;
    let registry = cronos_network::state::Registry::pda().0;
    let snapshot = cronos_network::state::Snapshot::pda(0).0;

    let manager = cronos_scheduler::state::Manager::pda(authority).0;
    let fee = cronos_scheduler::state::Fee::pda(manager).0;
    let cycler_queue = cronos_scheduler::state::Queue::pda(manager, 0).0;
    let snapshot_queue = cronos_scheduler::state::Queue::pda(manager, 1).0;
    let snapshot_task = cronos_scheduler::state::Task::pda(snapshot_queue, 0).0;

    Instruction {
        program_id: cronos_network::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(authority, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(config, false),
            AccountMeta::new(cycler, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(registry, false),
            AccountMeta::new_readonly(cronos_scheduler::ID, false),
            AccountMeta::new(snapshot, false),
            AccountMeta::new_readonly(system_program::ID, false),
            // Additional accounts
            AccountMeta::new(cycler_queue, false),
            AccountMeta::new(fee, false),
            AccountMeta::new(manager, false),
            AccountMeta::new(snapshot_task, false),
            AccountMeta::new(snapshot_queue, false),
        ],
        data: cronos_network::instruction::Initialize {}.data(),
    }
}
