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
    api: Pubkey,
    caller: Pubkey,
    id: String,
    method: HttpMethod,
    payer: Pubkey,
    route: String,
) -> Instruction {
    let config_pubkey = cronos_http::state::Config::pubkey();
    let pool_pubkey = cronos_pool::state::Pool::pubkey();
    let request_pubkey = cronos_http::state::Request::pubkey(api, caller, id.clone());
    Instruction {
        program_id: cronos_http::ID,
        accounts: vec![
            AccountMeta::new_readonly(api, false),
            AccountMeta::new_readonly(caller, true),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config_pubkey, false),
            AccountMeta::new(payer, true),
            AccountMeta::new(pool_pubkey, false),
            AccountMeta::new(request_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_http::instruction::RequestNew { id, method, route }.data(),
    }
}
