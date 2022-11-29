use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    clockwork_network_program::state::*,
    spl_associated_token_account::get_associated_token_address,
};

pub fn delegation_deposit(
    amount: u64,
    authority: Pubkey,
    delegation: Pubkey,
    mint: Pubkey,
) -> Instruction {
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(get_associated_token_address(&authority, &mint), false),
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new(delegation, false),
            AccountMeta::new(get_associated_token_address(&delegation, &mint), false),
            AccountMeta::new_readonly(anchor_spl::token::ID, false),
        ],
        data: clockwork_network_program::instruction::DelegationDeposit { amount }.data(),
    }
}
