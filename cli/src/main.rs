#[macro_use]
extern crate version;

mod cli;
mod config;
mod errors;
mod parser;
mod processor;

use cli::app;
use errors::CliError;
use processor::process;

fn main() -> Result<(), CliError>{
    process(&app().get_matches()).map_err(|e|{
        println!("{}", e);
        e
    })
}
