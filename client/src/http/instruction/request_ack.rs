use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn request_ack(ack_authority: Pubkey, request_pubkey: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_http::ID,
        accounts: vec![
            AccountMeta::new(ack_authority, true),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(request_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_http::instruction::RequestAck {}.data(),
    }
}
