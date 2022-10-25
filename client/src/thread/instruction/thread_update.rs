use clockwork_thread_program::objects::ThreadSettings;

use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn thread_update(authority: Pubkey, thread: Pubkey, settings: ThreadSettings) -> Instruction {
    Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(thread, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_thread_program::instruction::ThreadUpdate { settings }.data(),
    }
}
