/**
 * Program level
 */

pub mod queue_program {
    pub use clockwork_queue_program::{cpi, errors, program::QueueProgram, ID};
    pub mod accounts {
        pub use clockwork_queue_program::accounts::*;
        pub use clockwork_queue_program::objects::*;
    }
    pub mod utils {
        pub use clockwork_queue_program::utils::*;
        pub use clockwork_utils::*;
    }
}

/**
 * Client level
 */
#[cfg(feature = "client")]
pub use clockwork_client::{Client, ClientError, ClientResult, SplToken};
