#[macro_use]
extern crate version;

mod cli;
mod config;
mod parser;
mod processor;

use cli::{app, CliError};
use processor::process;

fn main() -> Result<(), CliError> {
    process(&app().get_matches())
}
