use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn node_drop_pool(authority: Pubkey, node: Pubkey, pool: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_network::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(node, false),
            AccountMeta::new_readonly(pool, false),
        ],
        data: clockwork_network::instruction::NodeDropPool {}.data(),
    }
}
