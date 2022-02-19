use anchor_client::anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn daemon_widthdraw(daemon: Pubkey, owner: Pubkey, amount: u64) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(daemon, false),
            AccountMeta::new(owner, true),
        ],
        data: cronos_program::instruction::DaemonWidthdraw { amount }.data(),
    }
}
