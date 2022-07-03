use anchor_lang::prelude::*;

#[error_code]
pub enum CronosError {
    #[msg("This http method is unrecognized")]
    InvalidHttpMethod,
}
