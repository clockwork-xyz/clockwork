use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    clockwork_network_program::objects::{Fee, Penalty, Pool},
};

pub fn queue_crank(
    data_hash: Option<u64>,
    queue: Pubkey,
    signatory: Pubkey,
    worker: Pubkey,
) -> Instruction {
    Instruction {
        program_id: clockwork_queue_program::ID,
        accounts: vec![
            AccountMeta::new(Fee::pubkey(worker), false),
            AccountMeta::new(Penalty::pubkey(worker), false),
            AccountMeta::new_readonly(Pool::pubkey(0), false),
            AccountMeta::new(queue, false),
            AccountMeta::new(signatory, true),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_queue_program::instruction::QueueCrank { data_hash }.data(),
    }
}
