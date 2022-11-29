use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program, sysvar,
        },
        InstructionData,
    },
    anchor_spl::{associated_token, associated_token::get_associated_token_address, token},
    clockwork_network_program::state::*,
};

pub fn worker_create(
    authority: Pubkey,
    mint: Pubkey,
    signatory: Pubkey,
    worker: Pubkey,
) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(associated_token::ID, false),
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new(Fee::pubkey(worker), false),
            AccountMeta::new(Penalty::pubkey(worker), false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(Registry::pubkey(), false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(signatory, true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(token::ID, false),
            AccountMeta::new(worker, false),
            AccountMeta::new(get_associated_token_address(&worker, &mint), false),
        ],
        data: clockwork_network_program::instruction::WorkerCreate {}.data(),
    }
}
