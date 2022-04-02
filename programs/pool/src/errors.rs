use anchor_lang::prelude::*;

#[error_code]
pub enum CronosError {
    #[msg("This account has already been initialized")]
    AccountAlreadyInitialized,

    #[msg("This instruction requires admin authority")]
    AdminAuthorityInvalid,

    #[msg("The registry is locked and may not be changed")]
    RegistryLocked,

    #[msg("The registry must be locked for this operation to be executed")]
    RegistryMustBeLocked,

    #[msg("The page is full")]
    PageIsFull,

    #[msg("The page does not contain the needed value")]
    PageRangeInvalid,

    #[msg("The snapshot has already been captured and is not currently in progress")]
    SnapshotNotInProgress,
    
    #[msg("The snapshot is not current")]
    SnapshotNotCurrent,

    #[msg("The snapshot is incomplete and has more to capture")]
    SnapshotIncomplete,

    #[msg("A new ")]
    SnapshotPage,

}


#[error_code]
pub enum SnapshotError {
    #[msg("The registry page int does not match the current state of the snapshot")]
    InvalidRegistryPage,

    
}

#[error_code]
pub enum PoolError {
    
    #[msg("The provided snapshot page doesn't hold the sampled node")]
    InvalidSnapshotPage,


}
