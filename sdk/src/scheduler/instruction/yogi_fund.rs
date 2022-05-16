use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn yogi_fund(amount: u64, payer: Pubkey, yogi: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(yogi, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::YogiFund { amount }.data(),
    }
}
