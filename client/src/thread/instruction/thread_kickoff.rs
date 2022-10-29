use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn thread_kickoff(signatory: Pubkey, thread: Pubkey, worker: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new(signatory, true),
            AccountMeta::new(thread, false),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_thread_program::instruction::ThreadKickoff {}.data(),
    }
}
