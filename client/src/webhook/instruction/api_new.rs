use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn api_new(
    ack_authority: Pubkey,
    authority: Pubkey,
    base_url: String,
    payer: Pubkey,
) -> Instruction {
    let api_pubkey = clockwork_webhook_program::objects::Api::pubkey(authority, base_url.clone());
    Instruction {
        program_id: clockwork_webhook_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(ack_authority, false),
            AccountMeta::new(api_pubkey, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_webhook_program::instruction::ApiNew { base_url }.data(),
    }
}
