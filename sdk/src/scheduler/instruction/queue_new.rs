use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        },
        InstructionData,
    },
    cronos_scheduler::pda::PDA,
};

pub fn queue_new(fee_pda: PDA, owner: Pubkey, queue_pda: PDA) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(fee_pda.0, false),
            AccountMeta::new(owner, true),
            AccountMeta::new(queue_pda.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_scheduler::instruction::QueueNew {
            fee_bump: fee_pda.1,
            queue_bump: queue_pda.1,
        }
        .data(),
    }
}
