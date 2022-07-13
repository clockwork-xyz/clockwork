use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};
use cronos_http::state::HttpMethod;

pub fn request_new(
    ack_authority: Pubkey,
    id: u128,
    method: HttpMethod,
    payer: Pubkey,
    url: String,
) -> Instruction {
    let config_pubkey = cronos_http::state::Config::pubkey();
    let manager_pubkey = cronos_http::state::Manager::pubkey(payer);
    let request_pubkey = cronos_http::state::Request::pubkey(manager_pubkey, id);
    Instruction {
        program_id: cronos_http::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config_pubkey, false),
            AccountMeta::new(manager_pubkey, false),
            AccountMeta::new(payer, true),
            AccountMeta::new(request_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_http::instruction::RequestNew {
            ack_authority,
            method,
            url,
        }
        .data(),
    }
}
