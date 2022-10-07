use anchor_lang::prelude::*;

#[error_code]
pub enum ClockworkError {
    #[msg("This account has already been initialized")]
    AccountAlreadyInitialized,

    #[msg("The commission must be between 0 and 100")]
    InvalidCommission,

    #[msg("The provided node is cannot be used for this operation")]
    InvalidWorker,

    #[msg("The provided snapshot entry cannot be used for this operation")]
    InvalidSnapshotEntry,

    #[msg("The stake account cannot be used for this operation")]
    InvalidStakeAccount,

    #[msg("The snapshot has reached an invalid state. This should not happen.")]
    InvalidSnapshot,

    #[msg("One of the provided pool accounts is invalid or missing")]
    InvalidPool,

    #[msg("You cannot request to unstake more tokens than are currently locked")]
    InvalidUnstakeAmount,

    #[msg("The registry is locked and may not be updated right now")]
    RegistryLocked,

    #[msg("The registry must be locked for this operation")]
    RegistryMustBeLocked,

    #[msg("The snapshot is not archived")]
    SnapshotNotArchived,

    #[msg("The snapshot is not in progress")]
    SnapshotNotInProgress,

    #[msg("The snapshot is not current")]
    SnapshotNotCurrent,

    #[msg("The snapshot is incomplete")]
    SnapshotIncomplete,
}
