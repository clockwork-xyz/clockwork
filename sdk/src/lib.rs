/**
 * Client level
 */

#[cfg(feature = "client")]
pub use clockwork_client::Client;

#[cfg(feature = "client")]
pub use clockwork_client::queue::*;

/**
 * Program level
 */

#[cfg(not(feature = "client"))]
pub use clockwork_queue_program::*;
