use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program, sysvar,
        },
        InstructionData,
    },
    clockwork_network_program::state::*,
    spl_associated_token_account::get_associated_token_address,
};

pub fn delegation_create(
    authority: Pubkey,
    delegation: Pubkey,
    mint: Pubkey,
    worker: Pubkey,
) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(anchor_spl::associated_token::ID, false),
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new(delegation, false),
            AccountMeta::new(get_associated_token_address(&delegation, &mint), false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(anchor_spl::token::ID, false),
            AccountMeta::new(worker, false),
        ],
        data: clockwork_network_program::instruction::DelegationCreate {}.data(),
    }
}
