pub mod network;
pub mod automation;
pub mod webhook;

mod client;
pub use client::{Client, ClientError, ClientResult, SplToken};
