use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn request_ack(ack_authority: Pubkey, close_to: Pubkey, request_pubkey: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_http::ID,
        accounts: vec![
            AccountMeta::new(ack_authority, true),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(close_to, false),
            AccountMeta::new(request_pubkey, false),
        ],
        data: cronos_http::instruction::RequestAck {}.data(),
    }
}
