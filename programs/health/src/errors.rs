use anchor_lang::prelude::*;

#[error_code]
pub enum CronosError {
    #[msg("This instruction requires admin authority")]
    NotAuthorizedAdmin,
}
