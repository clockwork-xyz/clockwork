use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey) -> Instruction {
    let config_pubkey = clockwork_pool_program::state::Config::pubkey();
    let pool_authority = clockwork_network_program::state::Rotator::pubkey();

    Instruction {
        program_id: clockwork_pool_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_pool_program::instruction::Initialize { pool_authority }.data(),
    }
}
