use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    clockwork_network_program::objects::*,
};

pub fn pool_create(
    admin: Pubkey,
    name: String,
    payer: Pubkey,
    pool: Pubkey,
    size: usize,
) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(admin, true),
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new(payer, true),
            AccountMeta::new(pool, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network_program::instruction::PoolCreate { name, size }.data(),
    }
}
