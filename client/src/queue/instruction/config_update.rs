use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    clockwork_queue::state::ConfigSettings,
};

pub fn config_update(admin: Pubkey, config: Pubkey, settings: ConfigSettings) -> Instruction {
    Instruction {
        program_id: clockwork_queue::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
        ],
        data: clockwork_queue::instruction::ConfigUpdate { settings }.data(),
    }
}
