use anchor_client::anchor_lang::InstructionData;
use cronos_program::state::InstructionData as CronosInstructionData;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub fn daemon_invoke(daemon: Pubkey, owner: Pubkey, instruction: Instruction) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(daemon, false),
            AccountMeta::new(owner, true),
        ],
        data: cronos_program::instruction::DaemonInvoke {
            instruction_data: CronosInstructionData::from(instruction),
        }
        .data(),
    }
}
