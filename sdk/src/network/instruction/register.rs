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
};

pub fn register(
    config: Pubkey,
    identity: Pubkey,
    mint: Pubkey,
    node: Pubkey,
    registry: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_network::ID,
        accounts: vec![
            AccountMeta::new_readonly(associated_token::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(identity, true),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(node, false),
            AccountMeta::new(registry, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(token::ID, false),
            AccountMeta::new(get_associated_token_address(&node, &mint), false),
        ],
        data: cronos_network::instruction::Register {}.data(),
    }
}
