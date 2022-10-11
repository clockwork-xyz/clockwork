use anchor_lang::prelude::*;

#[error_code]
pub enum ClockworkError {
    #[msg("The commission must be between 0 and 100")]
    InvalidCommission,

    #[msg("You cannot request to unstake more tokens than are currently locked")]
    InvalidUnstakeAmount,

    #[msg("The penalty account has an insufficient balance for this operation")]
    InsufficientPenaltyBalance,

    #[msg("The registry is locked and may not be updated right now")]
    RegistryLocked,
}
