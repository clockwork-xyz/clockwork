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
    authority: Pubkey,
    config: Pubkey,
    delegate: Pubkey,
    entry: Pubkey,
    mint: Pubkey,
    node: Pubkey,
    owner: Pubkey,
    registry: Pubkey,
    snapshot: Pubkey,
    snapshot_queue: Pubkey,
    snapshot_task: Pubkey,
    worker: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_network::ID,
        accounts: vec![
            AccountMeta::new_readonly(associated_token::ID, false),
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new(delegate, false),
            AccountMeta::new(entry, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(node, false),
            AccountMeta::new(owner, true),
            AccountMeta::new(registry, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(cronos_scheduler::ID, false),
            AccountMeta::new(snapshot, false),
            AccountMeta::new(snapshot_queue, false),
            AccountMeta::new(get_associated_token_address(&node, &mint), false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(token::ID, false),
            AccountMeta::new_readonly(worker, true),
            // Additional accounts
            AccountMeta::new(snapshot_task, false),
        ],
        data: cronos_network::instruction::NodeRegister {}.data(),
    }
}
