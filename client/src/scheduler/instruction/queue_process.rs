use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn queue_process(queue: Pubkey, worker: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_scheduler::ID,
        accounts: vec![
            AccountMeta::new(queue, false),
            AccountMeta::new(worker, true),
        ],
        data: clockwork_scheduler::instruction::QueueProcess {}.data(),
    }
}
