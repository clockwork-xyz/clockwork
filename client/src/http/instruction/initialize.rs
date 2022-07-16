use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey) -> Instruction {
    let config_pubkey = cronos_http::state::Config::pubkey();
    Instruction {
        program_id: cronos_http::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_http::instruction::Initialize {}.data(),
    }
}
