use anchor_lang::prelude::*;

#[error_code]
pub enum CronosError {
    #[msg("This account is already open")]
    AccountAlreadyOpen,

    #[msg("This instruction requires admin authority")]
    NotAuthorizedAdmin,

    #[msg("Unknown error")]
    Unknown,
}
