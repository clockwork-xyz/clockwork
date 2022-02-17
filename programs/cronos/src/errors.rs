use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Tasks cannot be started before they are stopped")]
    InvalidChronology,
    #[msg("Tasks cannot be scheduled for execution in the past")]
    InvalidExecAtStale,
    #[msg("Recurrence interval cannot be negative")]
    InvalidRecurrNegative,
    #[msg("Recurrence interval is below the minimum supported time granulartiy")]
    InvalidRecurrBelowMin,
    #[msg("Your daemon cannot provide all required signatures for this instruction")]
    InvalidSignatory,
    #[msg("Task is not queued and may not executed")]
    TaskNotQueued,
    #[msg("This task is not due and may not be executed yet")]
    TaskNotDue,
    #[msg("Unknown error")]
    Unknown,
}
