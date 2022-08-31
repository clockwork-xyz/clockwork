use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey) -> Instruction {
    let config_pubkey = clockwork_pool::state::Config::pubkey();
    let pool_authority = clockwork_network::state::Rotator::pubkey();

    Instruction {
        program_id: clockwork_pool::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_pool::instruction::Initialize { pool_authority }.data(),
    }
}
