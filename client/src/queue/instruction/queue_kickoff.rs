use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn queue_kickoff(
    data_hash: Option<u64>,
    queue: Pubkey,
    signatory: Pubkey,
    worker: Pubkey,
) -> Instruction {
    Instruction {
        program_id: clockwork_queue_program::ID,
        accounts: vec![
            AccountMeta::new(queue, false),
            AccountMeta::new(signatory, true),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_queue_program::instruction::QueueKickoff { data_hash }.data(),
    }
}
