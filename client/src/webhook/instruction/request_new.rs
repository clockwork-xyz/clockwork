use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};
use clockwork_webhook_program::objects::HttpMethod;

pub fn request_new(
    api: Pubkey,
    caller: Pubkey,
    id: String,
    method: HttpMethod,
    payer: Pubkey,
    route: String,
) -> Instruction {
    let config_pubkey = clockwork_webhook_program::objects::Config::pubkey();
    let pool_pubkey = clockwork_pool_program::objects::Pool::pubkey("http_workers".into());
    let request_pubkey =
        clockwork_webhook_program::objects::Request::pubkey(api, caller, id.clone());
    Instruction {
        program_id: clockwork_webhook_program::ID,
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
        data: clockwork_webhook_program::instruction::RequestNew { id, method, route }.data(),
    }
}
