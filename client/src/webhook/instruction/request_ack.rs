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
    caller: Pubkey,
    request: Pubkey,
    worker: Pubkey,
) -> Instruction {
    let config_pubkey = clockwork_webhook_program::objects::Config::pubkey();
    let fee_pubkey = clockwork_webhook_program::objects::Fee::pubkey(worker);
    Instruction {
        program_id: clockwork_webhook_program::ID,
        accounts: vec![
            AccountMeta::new(ack_authority, true),
            AccountMeta::new(caller, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config_pubkey, false),
            AccountMeta::new(fee_pubkey, false),
            AccountMeta::new(request, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_webhook_program::instruction::RequestAck {}.data(),
    }
}
