use {
    anchor_client::anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    cronos_scheduler::pda::PDA,
};

pub fn admin_initialize(
    admin: Pubkey,
    authority_pda: PDA,
    config_pda: PDA,
    daemon_pda: PDA,
    fee_pda: PDA,
) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(authority_pda.0, false),
            AccountMeta::new(config_pda.0, false),
            AccountMeta::new(daemon_pda.0, false),
            AccountMeta::new(fee_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::Initialize {
            authority_bump: authority_pda.1,
            config_bump: config_pda.1,
            daemon_bump: daemon_pda.1,
            fee_bump: fee_pda.1,
        }
        .data(),
    }
}
