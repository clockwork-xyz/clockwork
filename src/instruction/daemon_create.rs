use crate::pda::PDA;
use anchor_client::anchor_lang::InstructionData;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

pub fn daemon_create(daemon_pda: PDA, fee_pda: PDA, owner: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_program::ID,
        accounts: vec![
            AccountMeta::new(daemon_pda.0, false),
            AccountMeta::new(fee_pda.0, false),
            AccountMeta::new(owner, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_program::instruction::DaemonCreate {
            daemon_bump: daemon_pda.1,
            fee_bump: fee_pda.1,
        }
        .data(),
    }
}
