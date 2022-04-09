use {
    anchor_client::anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    cronos_heartbeat::pda::PDA,
};

pub fn initialize(admin: Pubkey, config_pda: PDA, heartbeat_pda: PDA) -> Instruction {
    Instruction {
        program_id: cronos_heartbeat::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config_pda.0, false),
            AccountMeta::new(heartbeat_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_heartbeat::instruction::Initialize {
            config_bump: config_pda.1,
            heartbeat_bump: heartbeat_pda.1,
        }
        .data(),
    }
}
