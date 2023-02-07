//! Errors thrown by the program.

use anchor_lang::prelude::*;

/// Errors for the the Clockwork automation program.
#[error_code]
pub enum ClockworkError {
    /// Thrown if a exec response has an invalid program ID or cannot be parsed.
    #[msg("The exec response could not be parsed")]
    InvalidAutomationResponse,

    /// Thrown if a automation has an invalid state and cannot complete the operation.
    #[msg("The automation is in an invalid state")]
    InvalidAutomationState,

    /// TThe provided trigger variant is invalid.
    #[msg("The trigger variant cannot be changed")]
    InvalidTriggerVariant,

    /// Thrown if a exec instruction is invalid because the automation's trigger condition has not been met.
    #[msg("The trigger condition has not been activated")]
    TriggerNotActive,

    #[msg("This operation cannot be processes because the automation is currently busy")]
    AutomationBusy,

    /// Thrown if a request is invalid because the automation is currently paused.
    #[msg("The automation is currently paused")]
    AutomationPaused,

    /// Thrown if a exec instruction would cause a automation to exceed its rate limit.
    #[msg("The automation's rate limit has been reached")]
    RateLimitExeceeded,

    /// Thrown if a automation authority attempts to set a rate limit above the maximum allowed value.
    #[msg("Automation rate limits cannot exceed the maximum allowed value")]
    MaxRateLimitExceeded,

    /// Thrown if an inner instruction attempted to write to an unauthorized address.
    #[msg("Inner instruction attempted to write to an unauthorized address")]
    UnauthorizedWrite,

    /// Thrown if the user attempts to withdraw SOL that would put a automation below it's minimum rent threshold.
    #[msg("Withdrawing this amount would leave the automation with less than the minimum required SOL for rent exemption")]
    WithdrawalTooLarge,
}
