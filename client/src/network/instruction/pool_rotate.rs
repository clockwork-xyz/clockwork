use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    clockwork_network_program::state::*,
};

pub fn pool_rotate(
    pool: Pubkey,
    signatory: Pubkey,
    snapshot: Pubkey,
    snapshot_frame: Pubkey,
    worker: Pubkey,
) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new(pool, false),
            AccountMeta::new_readonly(Registry::pubkey(), false),
            AccountMeta::new(signatory, true),
            AccountMeta::new_readonly(snapshot, false),
            AccountMeta::new_readonly(snapshot_frame, false),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_network_program::instruction::PoolRotate {}.data(),
    }
}
