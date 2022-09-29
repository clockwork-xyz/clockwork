use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        system_program, InstructionData,
    },
    clockwork_crank::state::{Config, Fee},
    clockwork_pool::state::Pool,
};

pub fn queue_crank(data_hash: Option<u64>, queue: Pubkey, worker: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_crank::ID,
        accounts: vec![
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new(Fee::pubkey(worker), false),
            AccountMeta::new_readonly(Pool::pubkey("crank".into()), false),
            AccountMeta::new(queue, false),
            AccountMeta::new(system_program::ID, false),
            AccountMeta::new(worker, true),
        ],
        data: clockwork_crank::instruction::QueueCrank { data_hash }.data(),
    }
}
