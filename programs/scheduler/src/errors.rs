use anchor_lang::prelude::*;

#[error_code]
pub enum ClockworkError {
    #[msg("Worker addresses cannot be initialized accounts")]
    WorkerDataNotEmpty,

    #[msg("An task's inner ix failed to execute")]
    InnerIxFailed,
    #[msg("An inner instructure wants to mutate state owned by the scheduler")]
    InnerIxReentrancy,

    #[msg("The queue is current executing another task")]
    InvalidTask,
    #[msg("The dynamic account list is not the expect size")]
    InvalidDynamicAccounts,
    #[msg("The task response value could not be parsed")]
    InvalidTaskResponse,
    #[msg("The return data is intended for another program")]
    InvalidReturnData,
    #[msg("The cron expression is invalid")]
    InvalidSchedule,
    #[msg("Your queue cannot sign for all required signatures for this instruction")]
    InvalidSignatory,
    #[msg("The queue does not have the right status for this operation")]
    InvalidQueueStatus,

    #[msg("Your are not the admin authority")]
    NotAdmin,
    #[msg("You are not the authority of this queue")]
    NotQueueAuthority,

    #[msg("The queue does not have enough lamports for this operation")]
    InsufficientQueueBalance,
    #[msg("The queue is not due")]
    QueueNotDue,
}
