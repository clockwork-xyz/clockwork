//! Errors thrown by the program.

use anchor_lang::prelude::*;

/// Errors for the the Clockwork queue program.
#[error_code]
pub enum ClockworkError {
    /// Thrown if a crank instruction requires a `data_hash` argument and one was not provided by the worker.
    #[msg("This trigger requires a data hash observation")]
    DataHashNotPresent,

    /// Thrown if a crank response has an invalid program ID or cannot be parsed.
    #[msg("The crank response could not be parsed")]
    InvalidCrankResponse,

    /// Thrown if a queue has an invalid state and cannot complete the operation.
    #[msg("The queue is in an invalid state")]
    InvalidQueueState,

    /// Thrown if a crank instruction is invalid because the queue's trigger condition has not been met.
    #[msg("The trigger condition has not been activated")]
    TriggerNotActive,

    #[msg("This operation cannot be processes because the queue is currently busy")]
    QueueBusy,

    /// Thrown if a request is invalid because the queue is currently paused.
    #[msg("The queue is currently paused")]
    QueuePaused,

    /// Thrown if a crank instruction would cause a queue to exceed its rate limit.
    #[msg("The queue's rate limit has been reached")]
    RateLimitExeceeded,

    /// Thrown if a queue authority attempts to set a rate limit above the maximum allowed value.
    #[msg("Queue rate limits cannot exceed the maximum allowed value")]
    MaxRateLimitExceeded,

    /// Thrown if an inner instruction attempted to write to an unauthorized address.
    #[msg("Inner instruction attempted to write to an unauthorized address")]
    UnauthorizedWrite,
}
