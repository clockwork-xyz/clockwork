use {
    anchor_client::anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    cronos_scheduler::state::InstructionData as CronosInstructionData,
};

pub fn daemon_sign(daemon: Pubkey, owner: Pubkey, instruction: Instruction) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(daemon, false),
            AccountMeta::new(owner, true),
        ],
        data: cronos_scheduler::instruction::DaemonSign {
            ix: CronosInstructionData::from(instruction),
        }
        .data(),
    }
}
