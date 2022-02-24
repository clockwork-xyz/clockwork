use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn fee_collect(fee: Pubkey, signer: Pubkey, treasury: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(fee, false),
            AccountMeta::new(signer, true),
            AccountMeta::new(treasury, false),
        ],
        data: cronos_program::instruction::FeeCollect {}.data(),
    }
}
