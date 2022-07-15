use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn request_ack(
    ack_authority: Pubkey,
    close_to: Pubkey,
    request: Pubkey,
    worker: Pubkey,
) -> Instruction {
    let config_pubkey = cronos_http::state::Config::pubkey();
    let fee_pubkey = cronos_http::state::Fee::pubkey(worker);
    Instruction {
        program_id: cronos_http::ID,
        accounts: vec![
            AccountMeta::new(ack_authority, true),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(close_to, false),
            AccountMeta::new_readonly(config_pubkey, false),
            AccountMeta::new(fee_pubkey, false),
            AccountMeta::new(request, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(worker, false),
        ],
        data: cronos_http::instruction::RequestAck {}.data(),
    }
}
