use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn admin_update_admin(admin: Pubkey, config: Pubkey, new_admin: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
        ],
        data: cronos_program::instruction::AdminUpdateAdmin { new_admin }.data(),
    }
}
