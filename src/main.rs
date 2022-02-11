pub mod command;
pub mod error;

use crate::command::CliCommand;

use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("Cronos")
        .bin_name("cronos")
        .about("Cronos is an instruction scheduler for Solana")
        .version("0.0.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(daemon_app())
        .subcommand(task_app())
        .get_matches();

    let command = CliCommand::try_from(&matches);

    println!("The command: {:?}", command);
}

fn daemon_app() -> App<'static> {
    App::new("daemon")
        .about("Manage your daemon")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("data").about("Fetch daemon account data"))
        .subcommand(App::new("new").about("Create a daemon account"))
}

fn task_app() -> App<'static> {
    App::new("task")
        .about("Manage your tasks")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("data").about("Fetch task account data").arg(
                Arg::new("address")
                    .index(1)
                    .takes_value(true)
                    .required(true)
                    .help("A task address"),
            ),
        )
        .subcommand(App::new("new").about("Create a task"))
}
