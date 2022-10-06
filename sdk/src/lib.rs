pub use clockwork_utils::*;
/**
 * Program level
 */

#[cfg(feature = "queue")]
pub mod queue_program {
    pub use clockwork_queue_program::{cpi, errors, program::QueueProgram, utils, ID};
    pub mod accounts {
        pub use clockwork_queue_program::accounts::*;
        pub use clockwork_queue_program::objects::*;
    }
}

/**
 * Client level
 */
#[cfg(feature = "client")]
pub mod client {
    pub use clockwork_client::{
        queue as queue_program, Client, ClientError, ClientResult, SplToken,
    };
}
