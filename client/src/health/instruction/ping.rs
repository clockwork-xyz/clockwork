use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn ping(signer: Pubkey) -> Instruction {
    let health_pubkey = cronos_health::state::Health::pubkey();
    Instruction {
        program_id: cronos_health::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(health_pubkey, false),
            AccountMeta::new(signer, true),
        ],
        data: cronos_health::instruction::Ping {}.data(),
    }
}
