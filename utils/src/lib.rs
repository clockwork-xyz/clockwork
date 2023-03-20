pub mod explorer;
pub mod pubkey;
pub mod thread;

use std::fmt::{Debug, Display, Formatter};

use anchor_lang::{prelude::Pubkey, prelude::*, AnchorDeserialize};
use base64;

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
    ) -> std::result::Result<Self, ErrorCode>
    where
        Self: Sized;
}

impl<T> ProgramLogsDeserializable for T
where
    T: AnchorDeserialize,
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
