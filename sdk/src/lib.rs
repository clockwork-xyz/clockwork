// For everyone
pub use clockwork_utils::*;

// For clients
#[cfg(feature = "client")]
pub use clockwork_client::{queue as queue_program, Client, ClientError, ClientResult, SplToken};

// For programs that need to CPI into Clockwork.
#[cfg(feature = "queue")]
pub mod queue_program {
    pub use clockwork_queue_program::{cpi, errors, program::QueueProgram, ID};
    pub mod accounts {
        pub use clockwork_queue_program::accounts::*;
        pub use clockwork_queue_program::objects::*;
    }
}
