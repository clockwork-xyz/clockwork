//! Errors thrown by the program.

use anchor_lang::prelude::*;

/// Errors for the the Clockwork queue program.
#[error_code]
pub enum ClockworkError {
    /// Thrown if a crank response has an invalid program ID or cannot be parsed.
    #[msg("The crank response could not be parsed")]
    InvalidCrankResponse,

    /// Thrown if a queue has an invalid state.
    #[msg("The queue is in an invalid state")]
    InvalidQueueState,

    /// Thrown if a request is invalid because the queue's trigger condition has not been met.
    #[msg("The trigger condition has not been met")]
    InvalidTrigger,

    /// Thrown if a request is invalid because the queue is currently paused.
    #[msg("The queue is currently paused")]
    PausedQueue,

    /// Thrown if a request would cause a queue to exceed its rate limit.
    #[msg("The queue's rate limit has been reached")]
    RateLimitExeceeded,

    /// Thrown if a value provided for rate limit exceeds the maximum allowed value
    #[msg("The value provided for rate limit exceeds the maximum allowed value")]
    RateLimitTooLarge,

    /// Thrown if an inner instruction attempted to write to an unauthorized address.
    #[msg("Inner instruction attempted to write to an unauthorized address")]
    UnauthorizedWrite,
}
