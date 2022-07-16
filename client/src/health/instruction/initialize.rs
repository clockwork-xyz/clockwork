use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(signer: Pubkey) -> Instruction {
    let health_pubkey = cronos_health::state::Health::pubkey();
    Instruction {
        program_id: cronos_health::ID,
        accounts: vec![
            AccountMeta::new(health_pubkey, false),
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_health::instruction::Initialize {}.data(),
    }
}
