use {
    anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        system_program, InstructionData,
    },
    clockwork_network_program::objects::Pool,
};

pub fn queue_crank(data_hash: Option<u64>, queue: Pubkey, signatory: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_queue_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(Pool::pubkey(0), false),
            AccountMeta::new(queue, false),
            AccountMeta::new(signatory, true),
            AccountMeta::new(system_program::ID, false),
        ],
        data: clockwork_queue_program::instruction::QueueCrank { data_hash }.data(),
    }
}
