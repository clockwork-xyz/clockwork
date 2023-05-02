mod cli;
mod client;
mod config;
mod deps;
mod errors;
mod parser;
mod print;
mod processor;

use {
    crate::{
        config::CliConfig,
        print::print_style,
    },
    cli::app,
    errors::CliError,
    processor::process,
};

fn main() -> Result<(), CliError> {
    process(&app().get_matches()).map_err(|e| {
        print_error!("{}", e);
        e
    })
}
