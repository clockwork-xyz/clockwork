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
) -> Instruction {
    let signer_tokens = get_associated_token_address(&signer, &mint);
    let stake_pubkey = get_associated_token_address(&node, &mint);
    Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(config, false),
            AccountMeta::new_readonly(node, false),
            AccountMeta::new(stake_pubkey, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(token::ID, false),
            AccountMeta::new(signer_tokens, false),
        ],
        data: clockwork_network_program::instruction::NodeStake { amount }.data(),
    }
}
