use clap::{App, AppSettings};

pub fn cronos() -> App<'static> {
    App::new("Cronos")
        .bin_name("cronos")
        .about("Cronos is an instruction scheduler for Solana")
        .version("0.0.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(super::admin::app())
        .subcommand(super::blocktime::app())
        .subcommand(super::config::app())
        .subcommand(super::daemon::app())
        .subcommand(super::health::app())
        .subcommand(super::task::app())
}
