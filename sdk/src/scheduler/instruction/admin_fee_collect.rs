use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn admin_fee_collect(admin: Pubkey, config: Pubkey, fee: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
            AccountMeta::new(fee, false),
        ],
        data: cronos_scheduler::instruction::AdminFeeCollect {}.data(),
    }
}
