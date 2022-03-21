use {
    anchor_client::anchor_lang::{
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        InstructionData,
    },
    cronos_program::state::InstructionData as CronosInstructionData
};

pub fn daemon_sign(daemon: Pubkey, owner: Pubkey, instruction: Instruction) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(daemon, false),
            AccountMeta::new(owner, true),
        ],
        data: cronos_program::instruction::DaemonSign {
            ix: CronosInstructionData::from(instruction),
        }
        .data(),
    }
}
