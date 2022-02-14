mod app;
mod command;
mod config;
mod error;
mod parser;
mod processor;
mod utils;

fn main() -> Result<(), crate::error::CliError> {
    let matches = crate::app::cronos().get_matches();
    crate::processor::process(&matches)
}
