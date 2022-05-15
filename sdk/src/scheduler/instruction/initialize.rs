use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey) -> Instruction {
    let authority = cronos_scheduler::state::Authority::pda().0;
    let config = cronos_scheduler::state::Config::pda().0;
    let queue = cronos_scheduler::state::Queue::pda(authority).0;
    let fee = cronos_scheduler::state::Fee::pda(queue).0;
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(authority, false),
            AccountMeta::new(config, false),
            AccountMeta::new(fee, false),
            AccountMeta::new(queue, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::Initialize {}.data(),
    }
}
