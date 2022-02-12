use clap::{App, AppSettings, Arg};

pub fn cronos_app() -> App<'static> {
    App::new("Cronos")
        .bin_name("cronos")
        .about("Cronos is an instruction scheduler for Solana")
        .version("0.0.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(daemon_app())
        .subcommand(health_app())
        .subcommand(task_app())
}

fn daemon_app() -> App<'static> {
    App::new("daemon")
        .about("Manage your daemon")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("data").about("Fetch daemon account data"))
        .subcommand(App::new("new").about("Create a daemon account"))
}

fn health_app() -> App<'static> {
    App::new("health")
        .about("Check the Cronos health")
        .subcommand(App::new("reset").about("Reset the Cronos health tracker"))
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
