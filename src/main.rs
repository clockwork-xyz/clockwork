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
        .get_matches();

    let command = CliCommand::try_from(&matches);

    println!("The command: {:?}", command);
}

fn daemon_app() -> App<'static> {
    App::new("daemon")
        .about("Manage your daemon account")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("data").about("Create a daemon account").arg(
                Arg::new("address")
                    .short('a')
                    .long("address")
                    .takes_value(true)
                    .help("A daemon address"),
            ),
        )
        .subcommand(App::new("new").about("Create a daemon account"))
}
