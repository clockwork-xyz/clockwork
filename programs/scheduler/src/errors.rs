use anchor_lang::prelude::*;

#[error_code]
pub enum CronosError {
    #[msg("The cron expression is invalid")]
    InvalidSchedule,
    #[msg("Your queue cannot provide all required signatures for this instruction")]
    InvalidSignatory,
    #[msg("The task does not have the right status for this operation")]
    InvalidTaskStatus,

    #[msg("Your are not the admin authority")]
    NotAdmin,
    #[msg("You are not the owner of this queue")]
    NotQueueOwner,

    #[msg("Task is not queued and may not executed")]
    TaskNotQueued,
    #[msg("This task is not due and may not be executed yet")]
    TaskNotDue,
    #[msg("The task instruction invocation failed")]
    TaskFailed,
}
