pub mod app;
pub mod command;
pub mod error;

use crate::app::cronos_app;
use crate::command::CliCommand;

fn main() {
    let matches = cronos_app().get_matches();
    let command = CliCommand::try_from(&matches);
    println!("The command: {:?}", command);
}
