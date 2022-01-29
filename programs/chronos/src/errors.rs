use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("This error does not have a name yet")]
    Unknown,
}
