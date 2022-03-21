use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn daemon_close(daemon: Pubkey, owner: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(daemon, false),
            AccountMeta::new(owner, true),
        ],
        data: cronos_program::instruction::DaemonClose {}.data(),
    }
}
