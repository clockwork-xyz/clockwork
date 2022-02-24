use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn admin_update_program_fee(
    admin: Pubkey,
    config: Pubkey,
    new_program_fee: u64,
) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
        ],
        data: cronos_program::instruction::AdminUpdateProgramFee { new_program_fee }.data(),
    }
}
