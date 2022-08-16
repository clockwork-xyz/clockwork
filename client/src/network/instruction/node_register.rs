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

pub fn node_register(
    config: Pubkey,
    entry: Pubkey,
    mint: Pubkey,
    node: Pubkey,
    owner: Pubkey,
    registry: Pubkey,
    snapshot: Pubkey,
    worker: Pubkey,
) -> Instruction {
    let stake_pubkey = get_associated_token_address(&node, &mint);
    Instruction {
        program_id: clockwork_network::ID,
        accounts: vec![
            AccountMeta::new_readonly(associated_token::ID, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(entry, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(node, false),
            AccountMeta::new(owner, true),
            AccountMeta::new(registry, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new(snapshot, false),
            AccountMeta::new(stake_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(token::ID, false),
            AccountMeta::new_readonly(worker, true),
        ],
        data: clockwork_network::instruction::NodeRegister {}.data(),
    }
}
