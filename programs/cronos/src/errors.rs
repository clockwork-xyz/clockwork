use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Your daemon cannot provide all required signatures for this instruction")]
    InvalidSignatory,
    #[msg("Tasks cannot be scheduled for execution in the past")]
    InvalidExecuteAt,
    #[msg("Task is not pending and may not executed")]
    TaskNotPending,
    #[msg("This task is not due and may not be executed yet")]
    TaskNotDue,
    #[msg("Unknown error")]
    Unknown,
}
