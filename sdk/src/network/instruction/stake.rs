use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    anchor_spl::{associated_token::get_associated_token_address, token},
};

pub fn stake(
    amount: u64,
    config: Pubkey,
    identity: Pubkey,
    mint: Pubkey,
    node: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_network::ID,
        accounts: vec![
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(identity, true),
            AccountMeta::new_readonly(node, false),
            AccountMeta::new(get_associated_token_address(&node, &mint), false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(token::ID, false),
            AccountMeta::new(get_associated_token_address(&identity, &mint), false),
        ],
        data: cronos_network::instruction::Stake { amount }.data(),
    }
}
