//! Clockwork errors

use anchor_lang::prelude::*;

#[error_code]
pub enum ClockworkError {
    #[msg("The crank response could not be parsed")]
    InvalidCrankResponse,
    #[msg("The queue is in an invalid state")]
    InvalidQueueState,
    #[msg("The provided rate limit is too large")]
    InvalidRateLimit,
    #[msg("The trigger condition has not been met")]
    InvalidTrigger,

    #[msg("The queue is currently paused")]
    PausedQueue,

    #[msg("The queue's rate limit has been reached")]
    RateLimitExeceeded,

    #[msg("Inner instruction attempted to write to an unauthorized address")]
    UnauthorizedWrite,
}
