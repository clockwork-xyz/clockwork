use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn api_new(ack_authority: Pubkey, base_url: String, payer: Pubkey) -> Instruction {
    let api_pubkey = cronos_http::state::Api::pubkey(base_url.clone(), payer);
    Instruction {
        program_id: cronos_http::ID,
        accounts: vec![
            AccountMeta::new_readonly(ack_authority, false),
            AccountMeta::new(api_pubkey, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_http::instruction::ApiNew { base_url }.data(),
    }
}
