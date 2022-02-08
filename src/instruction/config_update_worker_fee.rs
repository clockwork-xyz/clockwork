use anchor_client::anchor_lang::InstructionData;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub fn config_update_worker_fee(admin: Pubkey, config: Pubkey, new_worker_fee: u64) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
        ],
        data: cronos_program::instruction::ConfigUpdateWorkerFee { new_worker_fee }.data(),
    }
}
