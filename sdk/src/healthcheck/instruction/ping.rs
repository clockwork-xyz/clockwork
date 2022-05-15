use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn ping(signer: Pubkey) -> Instruction {
    let health = cronos_healthcheck::state::Health::pda().0;
    Instruction {
        program_id: cronos_healthcheck::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(health, false),
            AccountMeta::new(signer, true),
        ],
        data: cronos_healthcheck::instruction::Ping {}.data(),
    }
}
