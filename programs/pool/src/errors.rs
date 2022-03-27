use anchor_lang::prelude::*;

#[error_code]
pub enum AccountError {
    #[msg("This account has already been initialized")]
    AlreadyInitialized,
}

#[error_code]
pub enum AdminError {
    #[msg("This instruction requires admin authority")]
    NotAuthorized,
    
    #[msg("Unknown error")]
    Unknown,
}

#[error_code]
pub enum SnapshotError {
    #[msg("The registry page int does not match the current state of the snapshot")]
    InvalidRegistryPage,
    #[msg("The snapshot is already finished")]
    AlreadyDone,
}

