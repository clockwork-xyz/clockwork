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
}

#[error_code]
pub enum RegistryError {
    #[msg("The registry is locked")]
    Locked,

    #[msg("The registry page is full")]
    PageFull
}

#[error_code]
pub enum SnapshotError {
    #[msg("The registry page int does not match the current state of the snapshot")]
    InvalidRegistryPage,

    #[msg("The snapshot is already done")]
    AlreadyDone,
    #[msg("The snapshot is not done")]
    NotDone,
}

#[error_code]
pub enum PoolError {
    #[msg("There isn't a valid snapshot")]
    InvalidSnapshot,
    #[msg("The provided snapshot page doesn't hold the sampled node")]
    InvalidSnapshotPage,
}
