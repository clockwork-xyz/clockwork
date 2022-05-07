use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program, sysvar,
        },
        InstructionData,
    },
    cronos_network::pda::PDA,
};

pub fn initialize(
    admin: Pubkey,
    mint: Pubkey,
    config_pda: PDA,
    pool_pda: PDA,
    registry_pda: PDA,
    snapshot_pda: PDA,
) -> Instruction {
    Instruction {
        program_id: cronos_network::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(config_pda.0, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(pool_pda.0, false),
            AccountMeta::new(registry_pda.0, false),
            AccountMeta::new(snapshot_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_network::instruction::Initialize {
            config_bump: config_pda.1,
            pool_bump: pool_pda.1,
            registry_bump: registry_pda.1,
            snapshot_bump: snapshot_pda.1,
        }
        .data(),
    }
}
