use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Your daemon cannot sign for all required signatures on this instruction")]
    InvalidSignatory,
    #[msg("This error does not have a name yet")]
    Unknown,
}
