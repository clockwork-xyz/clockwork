use crate::pda::PDA;
use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(
    authority_pda: PDA,
    config_pda: PDA,
    daemon_pda: PDA,
    fee_pda: PDA,
    health_pda: PDA,
    treasury_pda: PDA,
    signer: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(authority_pda.0, false),
            AccountMeta::new(config_pda.0, false),
            AccountMeta::new(daemon_pda.0, false),
            AccountMeta::new(fee_pda.0, false),
            AccountMeta::new(health_pda.0, false),
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(treasury_pda.0, false),
        ],
        data: cronos_program::instruction::Initialize {
            authority_bump: authority_pda.1,
            config_bump: config_pda.1,
            daemon_bump: daemon_pda.1,
            fee_bump: fee_pda.1,
            health_bump: health_pda.1,
            treasury_bump: treasury_pda.1,
        }
        .data(),
    }
}
