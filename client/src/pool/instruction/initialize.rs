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
    let rotator_pubkey = clockwork_network::state::Rotator::pubkey();
    let pool_pubkey = clockwork_pool::state::Pool::pubkey();
    Instruction {
        program_id: clockwork_pool::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config_pubkey, false),
            AccountMeta::new(pool_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_pool::instruction::Initialize {
            rotator: rotator_pubkey,
        }
        .data(),
    }
}
