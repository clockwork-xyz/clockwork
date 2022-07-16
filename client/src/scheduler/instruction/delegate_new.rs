use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn delegate_new(authority: Pubkey, payer: Pubkey) -> Instruction {
    let delegate_pubkey = cronos_scheduler::state::Delegate::pubkey(authority);
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(delegate_pubkey, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::DelegateNew {}.data(),
    }
}
