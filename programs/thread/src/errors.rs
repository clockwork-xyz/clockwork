//! Errors thrown by the program.

use anchor_lang::prelude::*;

/// Errors for the the Clockwork thread program.
#[error_code]
pub enum ClockworkError {
    /// Thrown if a crank instruction requires a `data_hash` argument and one was not provided by the worker.
    #[msg("This trigger requires a data hash observation")]
    DataHashNotPresent,

    /// Thrown if a crank response has an invalid program ID or cannot be parsed.
    #[msg("The crank response could not be parsed")]
    InvalidCrankResponse,

    /// Thrown if a thread has an invalid state and cannot complete the operation.
    #[msg("The thread is in an invalid state")]
    InvalidThreadState,

    /// Thrown if an account trigger has an invalid range.
    #[msg("The range is larger than the account size")]
    RangeOutOfBounds,

    /// Thrown if a crank instruction is invalid because the thread's trigger condition has not been met.
    #[msg("The trigger condition has not been activated")]
    TriggerNotActive,

    #[msg("This operation cannot be processes because the thread is currently busy")]
    ThreadBusy,

    /// Thrown if a request is invalid because the thread is currently paused.
    #[msg("The thread is currently paused")]
    ThreadPaused,

    /// Thrown if a crank instruction would cause a thread to exceed its rate limit.
    #[msg("The thread's rate limit has been reached")]
    RateLimitExeceeded,

    /// Thrown if a thread authority attempts to set a rate limit above the maximum allowed value.
    #[msg("Thread rate limits cannot exceed the maximum allowed value")]
    MaxRateLimitExceeded,

    /// Thrown if an inner instruction attempted to write to an unauthorized address.
    #[msg("Inner instruction attempted to write to an unauthorized address")]
    UnauthorizedWrite,
}
