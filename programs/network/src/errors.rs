use anchor_lang::prelude::*;

#[error_code]
pub enum CronosError {
    #[msg("This account has already been initialized")]
    AccountAlreadyInitialized,

    #[msg("This instruction requires admin authority")]
    AdminAuthorityInvalid,

    #[msg("The provided node is cannot be used for this operation")]
    InvalidNode,

    #[msg("The provided snapshot entry cannot be used for this operation")]
    InvalidSnapshotEntry,

    #[msg("The stake account cannot be used for this operation")]
    InvalidStakeAccount,

    #[msg("The registry is locked and may not be updated right now")]
    RegistryLocked,

    #[msg("The registry must be locked for this operation")]
    RegistryMustBeLocked,

    #[msg("The snapshot is not in progress")]
    SnapshotNotInProgress,

    #[msg("The snapshot is not current")]
    SnapshotNotCurrent,

    #[msg("The snapshot is incomplete")]
    SnapshotIncomplete,
}
