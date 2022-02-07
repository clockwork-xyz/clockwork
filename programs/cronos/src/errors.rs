use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Your daemon cannot provide all required signatures for this instruction")]
    InvalidSignatory,
    #[msg("The provided timestamp is not a valid frame")]
    InvalidTimestamp,
    #[msg("Task is not pending and may not executed")]
    TaskNotPending,
    #[msg("This task is not marked for repeat")]
    TaskNotRepeatable,
    #[msg("This task is not due and may not be executed yet")]
    TaskNotDue,
    #[msg("Unknown error")]
    Unknown,
}
