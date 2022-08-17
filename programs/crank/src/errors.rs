use anchor_lang::prelude::*;

#[error_code]
pub enum ClockworkError {
    #[msg("Execution context is missing")]
    MissingExecContext,

    #[msg("An task's inner ix failed to execute")]
    InnerIxFailed,

    #[msg("The queue does not have enough lamports for this operation")]
    InsufficientQueueBalance,

    #[msg("The task response value could not be parsed")]
    InvalidCrankResponse,
    #[msg("The provided execution context is invalid")]
    InvalidExecContext,
    #[msg("The return data is intended for another program")]
    InvalidReturnData,
    #[msg("Your queue cannot sign for all required signatures for this instruction")]
    InvalidSignatory,

    #[msg("This queue has already started")]
    QueueAlreadyStarted,

    #[msg("Trigger condition has not been met")]
    TriggerNotMet,

    #[msg("This queue does not have a instruction to crank")]
    NoInstruction,

    #[msg("You are not the authority of this queue")]
    NotQueueAuthority,

    #[msg("Data cannot be saved at a worker address")]
    WorkerDataNotEmpty,
}
