use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn node_add_pool(authority: Pubkey, node: Pubkey, pool: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_network::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(node, false),
            AccountMeta::new_readonly(pool, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network::instruction::NodeAddPool {}.data(),
    }
}
