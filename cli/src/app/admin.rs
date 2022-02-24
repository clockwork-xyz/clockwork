use clap::{App, AppSettings, Arg};

pub fn app() -> App<'static> {
    App::new("admin")
        .about("Run admin instructions against Cronos")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(admin_cancel_app())
        .subcommand(admin_health_app())
        .subcommand(admin_initialize_app())
}

fn admin_cancel_app() -> App<'static> {
    App::new("cancel").about("Cancels a scheduled task").arg(
        Arg::new("address")
            .index(1)
            .takes_value(true)
            .help("A task address"),
    )
}

fn admin_health_app() -> App<'static> {
    App::new("health")
        .about("Admin health commands")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("reset").about("Resets the health account"))
        .subcommand(App::new("start").about("Starts a new health check"))
}

fn admin_initialize_app() -> App<'static> {
    App::new("initialize").about("Initializes the Cronos program")
}
