use anchor_lang::prelude::*;

#[error_code]
pub enum ClockworkError {
    #[msg("This instruction requires admin authority")]
    AdminAuthorityInvalid,

    #[msg("You cannot claim more than the collectable balance")]
    InvalidClaimAmount,

    #[msg("Http method is not recognized")]
    InvalidHttpMethod,

    #[msg("Invalid number of workers")]
    InvalidWorkers,
}
