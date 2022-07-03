use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};
use cronos_http::state::HttpMethod;

pub fn request_new(method: HttpMethod, payer: Pubkey, url: String) -> Instruction {
    let request_pubkey = cronos_http::state::Request::pubkey();
    Instruction {
        program_id: cronos_http::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(request_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_http::instruction::RequestNew { method, url }.data(),
    }
}
