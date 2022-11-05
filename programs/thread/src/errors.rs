//! Errors thrown by the program.

use anchor_lang::prelude::*;

/// Errors for the the Clockwork thread program.
#[error_code]
pub enum ClockworkError {
    /// Thrown if a exec response has an invalid program ID or cannot be parsed.
    #[msg("The exec response could not be parsed")]
    InvalidThreadResponse,

    /// Thrown if a thread has an invalid state and cannot complete the operation.
    #[msg("The thread is in an invalid state")]
    InvalidThreadState,

    /// Thrown if a exec instruction is invalid because the thread's trigger condition has not been met.
    #[msg("The trigger condition has not been activated")]
    TriggerNotActive,

    #[msg("This operation cannot be processes because the thread is currently busy")]
    ThreadBusy,

    /// Thrown if a request is invalid because the thread is currently paused.
    #[msg("The thread is currently paused")]
    ThreadPaused,

    /// Thrown if a exec instruction would cause a thread to exceed its rate limit.
    #[msg("The thread's rate limit has been reached")]
    RateLimitExeceeded,

    /// Thrown if a thread authority attempts to set a rate limit above the maximum allowed value.
    #[msg("Thread rate limits cannot exceed the maximum allowed value")]
    MaxRateLimitExceeded,

    /// Thrown if an inner instruction attempted to write to an unauthorized address.
    #[msg("Inner instruction attempted to write to an unauthorized address")]
    UnauthorizedWrite,

    /// Thrown if the user attempts to withdraw SOL that would put a thread below it's minimum rent threshold.
    #[msg("Withdrawing this amount would leave the thread with less than the minimum required SOL for rent exemption")]
    WithdrawalTooLarge,
}
