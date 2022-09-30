/**
 * Program level
 */

pub mod queue_program {
    pub use clockwork_queue_program::{
        accounts, cpi, errors, program::QueueProgram, state, utils, ID,
    };
}

/**
 * Client level
 */

#[cfg(feature = "client")]
pub use clockwork_client::{Client, ClientError, ClientResult};
