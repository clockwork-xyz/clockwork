use anchor_lang::prelude::*;

#[error_code]
pub enum CronosError {
    #[msg("This account has already been initialized")]
    AccountAlreadyInitialized,

    #[msg("This instruction requires admin authority")]
    AdminAuthorityInvalid,

    #[msg("The provided snapshot entry cannot be used for this operation")]
    InvalidSnapshotEntry,

    #[msg("The registry is locked and may not be changed")]
    RegistryLocked,

    #[msg("The registry must be locked for this operation to be executed")]
    RegistryMustBeLocked,

    #[msg("The snapshot has already been captured and is not currently in progress")]
    SnapshotNotInProgress,

    #[msg("The snapshot is not current")]
    SnapshotNotCurrent,

    #[msg("The snapshot is incomplete and has more to capture")]
    SnapshotIncomplete,

    #[msg("A new snapshot page is not needed")]
    SnapshotPageNotNeeded,
}
