use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Your daemon cannot sign for all required signatures on this instruction")]
    InvalidSignatory,
    #[msg("This task has already been executed")]
    TaskAlreadyExecuted,
    #[msg("This task has not come due yet")]
    TaskNotDue,
    #[msg("This error does not have a name yet")]
    Unknown,
}
