use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn queue_crank(exec: Pubkey, queue: Pubkey, worker: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_crank::ID,
        accounts: vec![
            AccountMeta::new(exec, false),
            AccountMeta::new(queue, false),
            AccountMeta::new(worker, true),
        ],
        data: clockwork_crank::instruction::QueueCrank {}.data(),
    }
}
