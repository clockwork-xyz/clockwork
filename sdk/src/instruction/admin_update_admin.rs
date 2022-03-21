use {
    anchor_client::anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    cronos_program::state::ConfigSettings
};

pub fn admin_config_update(admin: Pubkey, config: Pubkey, settings: ConfigSettings) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
        ],
        data: cronos_program::instruction::AdminConfigUpdate { settings }.data(),
    }
}
