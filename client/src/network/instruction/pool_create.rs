use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    system_program, InstructionData,
};

pub fn pool_create(admin: Pubkey, name: String, size: usize) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new_readonly(clockwork_network_program::objects::Config::pubkey(), false),
            AccountMeta::new(
                clockwork_pool_program::objects::Pool::pubkey(name.clone()),
                false,
            ),
            AccountMeta::new_readonly(clockwork_pool_program::ID, false),
            AccountMeta::new_readonly(clockwork_pool_program::objects::Config::pubkey(), false),
            AccountMeta::new(clockwork_network_program::objects::Rotator::pubkey(), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network_program::instruction::PoolCreate { name, size }.data(),
    }
}
