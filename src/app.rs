use clap::{App, AppSettings, Arg};

pub fn cronos_app() -> App<'static> {
    App::new("Cronos")
        .bin_name("cronos")
        .about("Cronos is an instruction scheduler for Solana")
        .version("0.0.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(blocktime_app())
        .subcommand(daemon_app())
        .subcommand(health_app())
        .subcommand(task_app())
}

fn blocktime_app() -> App<'static> {
    App::new("blocktime").about("Check the current Solana blocktime")
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
        .subcommand(task_new_app())
        .subcommand(
            App::new("data").about("Fetch task account data").arg(
                Arg::new("address")
                    .index(1)
                    .takes_value(true)
                    .required(true)
                    .help("A task address"),
            ),
        )
}

fn task_new_app() -> App<'static> {
    App::new("new")
        .about("Schedule a new task")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::new("exec_at")
                .long("exec_at")
                .short('e')
                .required(false)
                .global(true)
                .help("When to invoke the instruction"),
        )
        .arg(
            Arg::new("stop_at")
                .long("stop_at")
                .short('s')
                .required(false)
                .global(true)
                .help("When to stop invoke the instruction"),
        )
        .arg(
            Arg::new("recurr")
                .long("recurr")
                .short('r')
                .required(false)
                .global(true)
                .help("The duration to wait between instruction invocations"),
        )
        .subcommand(memo_app())
}

fn memo_app() -> App<'static> {
    App::new("memo")
        .about("Schedule memos with the SPL Memo program")
        .arg(
            Arg::new("memo")
                .index(1)
                .takes_value(true)
                .required(true)
                .help("The memo to write"),
        )
}
