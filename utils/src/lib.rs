pub mod explorer;

use {
    anchor_lang::{
        prelude::borsh::BorshSchema,
        prelude::Pubkey,
        prelude::*,
        solana_program::{self, instruction::Instruction},
        AnchorDeserialize,
    },
    static_pubkey::static_pubkey,
    std::{
        fmt::{Debug, Display, Formatter},
        convert::TryFrom,
        hash::Hash,
    },
    base64,
};

/// The stand-in pubkey for delegating a payer address to a worker. All workers are re-imbursed by the user for lamports spent during this delegation.
pub static PAYER_PUBKEY: Pubkey = static_pubkey!("C1ockworkPayer11111111111111111111111111111");

/// The sighash of a named instruction in an Anchor program.
pub fn anchor_sighash(name: &str) -> [u8; 8] {
    let namespace = "global";
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );
    sighash
}

/// The clock object, representing a specific moment in time recorded by a Solana cluster.
#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct ClockData {
    /// The current slot.
    pub slot: u64,
    /// The timestamp of the first slot in this Solana epoch.
    pub epoch_start_timestamp: i64,
    /// The bank epoch.
    pub epoch: u64,
    /// The future epoch for which the leader schedule has most recently been calculated.
    pub leader_schedule_epoch: u64,
    /// Originally computed from genesis creation time and network time
    /// in slots (drifty); corrected using validator timestamp oracle as of
    /// timestamp_correction and timestamp_bounding features.
    pub unix_timestamp: i64,
}

impl From<Clock> for ClockData {
    fn from(clock: Clock) -> Self {
        ClockData {
            slot: clock.slot,
            epoch_start_timestamp: clock.epoch_start_timestamp,
            epoch: clock.epoch,
            leader_schedule_epoch: clock.leader_schedule_epoch,
            unix_timestamp: clock.unix_timestamp,
        }
    }
}

impl From<&ClockData> for Clock {
    fn from(clock: &ClockData) -> Self {
        Clock {
            slot: clock.slot,
            epoch_start_timestamp: clock.epoch_start_timestamp,
            epoch: clock.epoch,
            leader_schedule_epoch: clock.leader_schedule_epoch,
            unix_timestamp: clock.unix_timestamp,
        }
    }
}

impl TryFrom<Vec<u8>> for ClockData {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Ok(
            borsh::try_from_slice_with_schema::<ClockData>(data.as_slice())
                .map_err(|_err| ErrorCode::AccountDidNotDeserialize)?,
        )
    }
}

/// A response value target programs can return to update the thread.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct ThreadResponse {
    /// The kickoff instruction to use on the next triggering of the thread.
    /// If none, the kickoff instruction remains unchanged.
    pub kickoff_instruction: Option<InstructionData>,
    /// The next instruction to use on the next execution of the thread.
    pub next_instruction: Option<InstructionData>,
}

impl Default for ThreadResponse {
    fn default() -> Self {
        return Self {
            kickoff_instruction: None,
            next_instruction: None,
        };
    }
}

/// The data needed execute an instruction on Solana.
#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, Hash, PartialEq)]
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

/// Account metadata needed to execute an instruction on Solana.
#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, Hash, PartialEq)]
pub struct AccountMetaData {
    /// An account's public key
    pub pubkey: Pubkey,
    /// True if an Instruction requires a Transaction signature matching `pubkey`.
    pub is_signer: bool,
    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,
}

impl AccountMetaData {
    /// Construct metadata for a writable account.
    pub fn new(pubkey: Pubkey, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: true,
        }
    }

    /// Construct metadata for a read-only account.
    pub fn new_readonly(pubkey: Pubkey, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: false,
        }
    }
}

/// Crate build information
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct CrateInfo {
    /// The link to the crate spec
    pub spec: String,
    /// Arbitrary blob that can be set by developers
    pub blob: String,
}

impl Display for CrateInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "spec:{} blob:{}", self.spec, self.blob)
    }
}

/// Parse struct from sol_set_return_data in program's logs
pub trait ProgramLogsDeserializable {
    fn try_from_program_logs(
        program_logs: Vec<String>,
        program_id: &Pubkey,
    ) -> std::result::Result<Self, ErrorCode> where Self: Sized;
}

impl<T> ProgramLogsDeserializable for T
    where
        T: AnchorDeserialize
{
    fn try_from_program_logs(
        program_logs: Vec<String>,
        program_id: &Pubkey,
    ) -> std::result::Result<T, ErrorCode> {
        // A Program's return data appears in the log in this format:
        // "Program return: <program-id> <program-generated-data-in-base64>"
        // https://github.com/solana-labs/solana/blob/b8837c04ec3976c9c16d028fbee86f87823fb97f/program-runtime/src/stable_log.rs#L68
        let preimage = format!("Program return: {} ", program_id.to_string());

        // Extract the return data after Program return: <program-id>
        let get_return_data_base64 = program_logs
            .iter()
            .find(|&s| s.starts_with(&preimage))
            .ok_or(ErrorCode::AccountDidNotDeserialize)?
            .strip_prefix(&preimage)
            .ok_or(ErrorCode::AccountDidNotDeserialize)?;

        let decoded = base64::decode(get_return_data_base64)
            .map_err(|_err| ErrorCode::AccountDidNotDeserialize)?;

        T::try_from_slice(&decoded).map_err(|_err| ErrorCode::AccountDidNotDeserialize)
    }
}
