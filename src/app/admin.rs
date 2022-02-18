use clap::{App, AppSettings, Arg};

pub fn app() -> App<'static> {
    App::new("admin")
        .about("Run admin instructions against Cronos")
        .subcommand(cancel_task_app())
        .subcommand(health_app())
}

fn health_app() -> App<'static> {
    App::new("health")
        .about("Admin health commands")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("reset").about("Resets the health account"))
        .subcommand(App::new("start").about("Starts a new health check"))
}

fn cancel_task_app() -> App<'static> {
    App::new("cancel").about("Cancels a scheduled task").arg(
        Arg::new("address")
            .index(1)
            .takes_value(true)
            .help("A task address"),
    )
}
