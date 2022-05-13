use anchor_lang::prelude::*;

#[error_code]
pub enum CronosError {
    #[msg("Delegate addresses cannot be initialized accounts")]
    DelegateDataNotEmpty,

    #[msg("An action's inner ix failed to execute")]
    InnerIxFailed,
    #[msg("An inner instructure wants to mutate state owned by the scheduler")]
    InnerIxReentrancy,

    #[msg("The task is current executing another action")]
    InvalidAction,
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

    #[msg("The task is not due")]
    TaskNotDue,

    #[msg("The CPI response value could not be parsed")]
    UnknownResponse,
}
