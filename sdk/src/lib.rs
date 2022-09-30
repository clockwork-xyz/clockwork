/**
 * Client level
 */

#[cfg(feature = "client")]
pub use clockwork_client::{Client, ClientError, ClientResult};

#[cfg(feature = "client")]
pub use clockwork_queue_program::{
    anchor, errors, payer, queue_program, state, ID as QueueProgramId,
};

/**
 * Program level
 */
#[cfg(not(feature = "client"))]
pub use clockwork_queue_program::*;
