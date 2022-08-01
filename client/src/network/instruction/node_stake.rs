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

pub fn node_stake(
    amount: u64,
    config: Pubkey,
    node: Pubkey,
    mint: Pubkey,
    signer: Pubkey,
    worker: Pubkey,
) -> Instruction {
    Instruction {
        program_id: clockwork_network::ID,
        accounts: vec![
            AccountMeta::new_readonly(config, false),
            AccountMeta::new_readonly(node, false),
            AccountMeta::new(get_associated_token_address(&node, &mint), false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(token::ID, false),
            AccountMeta::new(get_associated_token_address(&signer, &mint), false),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_network::instruction::NodeStake { amount }.data(),
    }
}
