use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Your daemon cannot provide all required signatures for this instruction")]
    InvalidSignatory,
    #[msg("This task is no longer pending and may not be executed again")]
    TaskNotPending,
    #[msg("This task is not due and may not be executed yet")]
    TaskNotDue,
    #[msg("Unknown error")]
    Unknown,
}
