use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    clockwork_crank::state::ConfigSettings,
};

pub fn config_update(admin: Pubkey, config: Pubkey, settings: ConfigSettings) -> Instruction {
    Instruction {
        program_id: clockwork_crank::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
        ],
        data: clockwork_crank::instruction::ConfigUpdate { settings }.data(),
    }
}
