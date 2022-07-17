use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn queue_process(queue: Pubkey, worker: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(queue, false),
            AccountMeta::new(worker, true),
        ],
        data: cronos_scheduler::instruction::QueueProcess {}.data(),
    }
}
