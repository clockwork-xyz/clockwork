use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    clockwork_queue_program::objects::ConfigSettings,
};

pub fn config_update(admin: Pubkey, config: Pubkey, settings: ConfigSettings) -> Instruction {
    Instruction {
        program_id: clockwork_queue_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
        ],
        data: clockwork_queue_program::instruction::ConfigUpdate { settings }.data(),
    }
}
