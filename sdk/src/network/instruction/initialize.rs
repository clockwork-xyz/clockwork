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

    let queue = cronos_scheduler::state::Queue::pda(authority).0;
    let fee = cronos_scheduler::state::Fee::pda(queue).0;
    let cycler_task = cronos_scheduler::state::Task::pda(queue, 0).0;
    let snapshot_task = cronos_scheduler::state::Task::pda(queue, 1).0;
    let snapshot_action = cronos_scheduler::state::Action::pda(snapshot_task, 0).0;

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
            AccountMeta::new(cycler_task, false),
            AccountMeta::new(fee, false),
            AccountMeta::new(queue, false),
            AccountMeta::new(snapshot_action, false),
            AccountMeta::new(snapshot_task, false),
        ],
        data: cronos_network::instruction::Initialize {}.data(),
    }
}
