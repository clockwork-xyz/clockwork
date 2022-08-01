use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    clockwork_scheduler::state::ConfigSettings,
};

pub fn admin_config_update(admin: Pubkey, config: Pubkey, settings: ConfigSettings) -> Instruction {
    Instruction {
        program_id: clockwork_scheduler::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
        ],
        data: clockwork_scheduler::instruction::AdminConfigUpdate { settings }.data(),
    }
}
