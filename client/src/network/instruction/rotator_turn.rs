use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn rotator_turn(entry: Pubkey, signer: Pubkey, snapshot: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_network::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(cronos_network::state::Config::pubkey(), false),
            AccountMeta::new_readonly(entry, false),
            AccountMeta::new(cronos_pool::state::Pool::pubkey(), false),
            AccountMeta::new_readonly(cronos_pool::state::Config::pubkey(), false),
            AccountMeta::new_readonly(cronos_pool::ID, false),
            AccountMeta::new(cronos_network::state::Rotator::pubkey(), false),
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(snapshot, false),
        ],
        data: cronos_network::instruction::RotatorTurn {}.data(),
    }
}
