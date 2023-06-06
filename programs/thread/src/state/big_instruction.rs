use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use clockwork_utils::thread::SerializableAccount;

pub const SEED_BIG_INST: &[u8] = b"big-instruction";

#[account]
#[derive(Debug)]
pub struct BigInstruction {
    pub authority: Pubkey,
    pub id: Vec<u8>,
    pub program_id: Pubkey,
    pub accounts: Vec<SerializableAccount>,
    pub data: Vec<u8>,
    pub bump: u8,
}

impl BigInstruction {
    /// Derive the pubkey of a BigInstruction.
    pub fn pubkey(authority: Pubkey, program_id: Pubkey, id: Vec<u8>) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_BIG_INST, authority.as_ref(), program_id.as_ref(), id.as_slice()],
            &crate::ID,
        )
        .0
    }
}

// todo: implement partialeq
impl PartialEq for BigInstruction {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.program_id.eq(&other.program_id) && self.id.eq(&other.id)
    }
}

impl Eq for BigInstruction {}

/// Trait for reading and writing to BigInstruction.
pub trait BigInstructionAccount {
    /// Get the pubkey of the big instruction.
    fn pubkey(&self) -> Pubkey;
}

impl BigInstructionAccount for Account<'_, BigInstruction> {
    fn pubkey(&self) -> Pubkey {
        BigInstruction::pubkey(self.authority, self.program_id, self.id.clone())
    }
}