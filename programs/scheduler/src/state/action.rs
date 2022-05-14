use {
    super::Task,
    crate::{errors::CronosError, pda::PDA},
    anchor_lang::{
        prelude::borsh::BorshSchema, prelude::*, solana_program::instruction::Instruction,
        AnchorDeserialize,
    },
    std::convert::TryFrom,
};

pub const SEED_ACTION: &[u8] = b"action";

/**
 * Action
 */

#[account]
#[derive(Debug)]
pub struct Action {
    pub id: u128,
    pub ixs: Vec<InstructionData>,
    pub task: Pubkey,
}

impl Action {
    pub fn pda(task: Pubkey, id: u128) -> PDA {
        Pubkey::find_program_address(
            &[SEED_ACTION, task.as_ref(), id.to_be_bytes().as_ref()],
            &crate::ID,
        )
    }
}

impl TryFrom<Vec<u8>> for Action {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Action::try_deserialize(&mut data.as_slice())
    }
}

/**
 * ActionAccount
 */

pub trait ActionAccount {
    fn new(&mut self, ixs: Vec<InstructionData>, task: &mut Account<Task>) -> Result<()>;
}

impl ActionAccount for Account<'_, Action> {
    fn new(&mut self, ixs: Vec<InstructionData>, task: &mut Account<Task>) -> Result<()> {
        // Reject inner instructions if they have a signer other than the queue or delegate
        for ix in ixs.iter() {
            for acc in ix.accounts.iter() {
                if acc.is_signer {
                    require!(
                        acc.pubkey == task.queue || acc.pubkey == crate::delegate::ID,
                        CronosError::InvalidSignatory
                    );
                }
            }
        }

        // Save data
        self.id = task.action_count;
        self.ixs = ixs;
        self.task = task.key();

        // Increment the task's action count
        task.action_count = task.action_count.checked_add(1).unwrap();

        Ok(())
    }
}

/**
 * InstructionData
 */

#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct InstructionData {
    /// Pubkey of the instruction processor that executes this instruction
    pub program_id: Pubkey,
    /// Metadata for what accounts should be passed to the instruction processor
    pub accounts: Vec<AccountMetaData>,
    /// Opaque data passed to the instruction processor
    pub data: Vec<u8>,
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

impl TryFrom<Vec<u8>> for InstructionData {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Ok(
            borsh::try_from_slice_with_schema::<InstructionData>(data.as_slice())
                .map_err(|_err| ErrorCode::AccountDidNotDeserialize)?,
        )
    }
}

/**
 * AccountMetaData
 */

#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct AccountMetaData {
    /// An account's public key
    pub pubkey: Pubkey,
    /// True if an Instruction requires a Transaction signature matching `pubkey`.
    pub is_signer: bool,
    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,
}
