use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn ping(heartbeat: Pubkey, signer: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_heartbeat::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(heartbeat, false),
            AccountMeta::new(signer, true),
        ],
        data: cronos_heartbeat::instruction::Ping {}.data(),
    }
}
