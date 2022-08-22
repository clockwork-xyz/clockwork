use clap::{Arg, Command};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    Benchmark {
        count: u32,
        parallelism: f32,
        recurrence: u32,
    },
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Account not found: {0}")]
    AccountNotFound(String),
    #[error("Account data could not be parsed: {0}")]
    AccountDataNotParsable(String),
    #[error("Bad client: {0}")]
    BadClient(String),
    #[error("Bad parameter: {0}")]
    BadParameter(String),
    #[error("Command not recognized: {0}")]
    CommandNotRecognized(String),
    #[error("Data point could not be found")]
    DataNotFound,
    #[error("No signers were provided")]
    NoSigners,
    #[error("There was an error with the websocket client")]
    WebsocketError,
}

pub fn app() -> Command<'static> {
    Command::new("Clockwork")
        .bin_name("clockwork-test")
        .about("Stress testing tool for Clockwork")
        .version(version!())
        .arg_required_else_help(true)
        .arg(
            Arg::new("count")
                .long("count")
                .short('c')
                .takes_value(true)
                .required(true)
                .help("Number of queues to schedule in this test"),
        )
        .arg(
            Arg::new("parallelism")
                .long("parallelism")
                .short('p')
                .takes_value(true)
                .help("Percentage of queues to execute in parallel"),
        )
        .arg(
            Arg::new("recurrence")
                .long("recurrence")
                .short('r')
                .takes_value(true)
                .help("Repeat queues every second this many times"),
        )
}
