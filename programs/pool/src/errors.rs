use anchor_lang::prelude::*;

#[error_code]
pub enum ClockworkError {
    #[msg("This instruction requires admin authority")]
    NotAuthorizedAdmin,
}
