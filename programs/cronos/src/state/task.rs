use crate::pda::PDA;

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use solana_program::instruction::Instruction;
use std::convert::TryFrom;

pub const SEED_TASK: &[u8] = b"task";

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum TaskStatus {
    Cancelled,
    Done,
    Queued,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Cancelled => write!(f, "cancelled"),
            TaskStatus::Done => write!(f, "done"),
            TaskStatus::Queued => write!(f, "queued"),
        }
    }
}

#[account]
#[derive(Debug)]
pub struct Task {
    pub daemon: Pubkey,
    pub exec_at: i64, // Time to execute at
    pub int: u128,
    pub interval: i64, // Duration between exec
    pub ix: InstructionData,
    pub status: TaskStatus,
    pub stop_at: i64, // Stop executing at
    pub bump: u8,
}

impl Task {
    pub fn find_pda(daemon: Pubkey, id: u128) -> PDA {
        Pubkey::find_program_address(
            &[SEED_TASK, daemon.as_ref(), id.to_be_bytes().as_ref()],
            &crate::ID,
        )
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub struct InstructionData {
    /// Pubkey of the instruction processor that executes this instruction
    pub program_id: Pubkey,
    /// Metadata for what accounts should be passed to the instruction processor
    pub accounts: Vec<AccountMetaData>,
    /// Opaque data passed to the instruction processor
    pub data: Vec<u8>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub struct AccountMetaData {
    /// An account's public key
    pub pubkey: Pubkey,
    /// True if an Instruction requires a Transaction signature matching `pubkey`.
    pub is_signer: bool,
    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,
}

impl TryFrom<Vec<u8>> for Task {
    type Error = ProgramError;
    fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
        Task::try_deserialize(&mut data.as_slice())
    }
}

impl From<Instruction> for InstructionData {
    fn from(instruction: Instruction) -> Self {
        InstructionData {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(|a| AccountMetaData {
                    pubkey: a.pubkey,
                    is_signer: a.is_signer,
                    is_writable: a.is_writable,
                })
                .collect(),
            data: instruction.data,
        }
    }
}

impl From<&InstructionData> for Instruction {
    fn from(instruction: &InstructionData) -> Self {
        Instruction {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(|a| AccountMeta {
                    pubkey: a.pubkey,
                    is_signer: a.is_signer,
                    is_writable: a.is_writable,
                })
                .collect(),
            data: instruction.data.clone(),
        }
    }
}
