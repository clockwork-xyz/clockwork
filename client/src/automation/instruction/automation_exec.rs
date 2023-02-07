use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};
use clockwork_network_program::state::{Fee, Pool};

pub fn automation_exec(signatory: Pubkey, automation: Pubkey, worker: Pubkey) -> Instruction {
    Instruction {
        program_id: clockwork_automation_program::ID,
        accounts: vec![
            AccountMeta::new(Fee::pubkey(worker), false),
            AccountMeta::new_readonly(Pool::pubkey(0), false),
            AccountMeta::new(signatory, true),
            AccountMeta::new(automation, false),
            AccountMeta::new_readonly(worker, false),
        ],
        data: clockwork_automation_program::instruction::AutomationExec {}.data(),
    }
}
