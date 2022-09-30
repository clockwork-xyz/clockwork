/**
 * Client level
 */

#[cfg(feature = "client")]
pub use clockwork_client::{Client, ClientError, ClientResult};

#[cfg(feature = "client")]
pub mod queue_program {
    pub use clockwork_queue_program::{errors, state, utils, ID};
}

/**
 * Program level
 */

#[cfg(not(feature = "client"))]
pub mod queue_program {
    pub use clockwork_queue_program::{
        accounts, cpi, errors, program::QueueProgram, state, utils, ID,
    };
}
