use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey, config: Pubkey, authorized_cycler: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_delegate::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
            // AccountMeta::new(heartbeat, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_delegate::instruction::Initialize { authorized_cycler }.data(),
    }
}
