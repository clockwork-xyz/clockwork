use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey) -> Instruction {
    let config = cronos_heartbeat::state::Config::pda().0;
    let heartbeat = cronos_heartbeat::state::Heartbeat::pda().0;
    Instruction {
        program_id: cronos_heartbeat::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
            AccountMeta::new(heartbeat, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_heartbeat::instruction::Initialize {}.data(),
    }
}
