use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

// TODO

pub fn pools_rotate(
    entry: Pubkey,
    node: Pubkey,
    signer: Pubkey,
    snapshot: Pubkey,
    worker: Pubkey,
) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(clockwork_network_program::objects::Config::pubkey(), false),
            AccountMeta::new_readonly(entry, false),
            AccountMeta::new_readonly(node, false),
            AccountMeta::new_readonly(clockwork_pool_program::ID, false),
            AccountMeta::new_readonly(clockwork_pool_program::state::Config::pubkey(), false),
            AccountMeta::new(clockwork_network_program::objects::Rotator::pubkey(), false),
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(snapshot, false),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_network_program::instruction::PoolsRotate {}.data(),
    }
}
