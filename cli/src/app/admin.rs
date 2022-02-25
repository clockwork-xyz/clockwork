use clap::{Command, Arg};

pub fn app() -> Command<'static> {
    Command::new("admin")
        .about("Run admin instructions against Cronos")
        .subcommand_required(true)
        .subcommand(admin_cancel_app())
        .subcommand(admin_health_app())
        .subcommand(admin_initialize_app())
}

fn admin_cancel_app() -> Command<'static> {
    Command::new("cancel").about("Cancels a scheduled task").arg(
        Arg::new("address")
            .index(1)
            .takes_value(true)
            .help("A task address"),
    )
}

fn admin_health_app() -> Command<'static> {
    Command::new("health")
        .about("Admin health commands")
        .subcommand_required(true)
        .subcommand(Command::new("reset").about("Resets the health account"))
        .subcommand(Command::new("start").about("Starts a new health check"))
}

fn admin_initialize_app() -> Command<'static> {
    Command::new("initialize").about("Initializes the Cronos program")
}
