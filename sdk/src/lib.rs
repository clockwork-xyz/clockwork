/**
 * Client level
 */

#[cfg(feature = "client")]
pub use clockwork_client::{Client, ClientError, ClientResult};

#[cfg(feature = "client")]
mod queue_program {
    pub use clockwork_queue_program::{anchor, errors, payer, state, ID};
}

/**
 * Program level
 */
#[cfg(not(feature = "client"))]
mod queue_program {
    pub use clockwork_queue_program::*;
}
