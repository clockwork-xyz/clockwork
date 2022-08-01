use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn rotator_turn(
    entry: Pubkey,
    signer: Pubkey,
    snapshot: Pubkey,
    worker: Pubkey,
) -> Instruction {
    Instruction {
        program_id: clockwork_network::ID,
        accounts: vec![
            AccountMeta::new_readonly(clockwork_network::state::Config::pubkey(), false),
            AccountMeta::new_readonly(entry, false),
            AccountMeta::new(clockwork_pool::state::Pool::pubkey(), false),
            AccountMeta::new_readonly(clockwork_pool::state::Config::pubkey(), false),
            AccountMeta::new_readonly(clockwork_pool::ID, false),
            AccountMeta::new(clockwork_network::state::Rotator::pubkey(), false),
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(snapshot, false),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_network::instruction::RotatorTurn {}.data(),
    }
}
