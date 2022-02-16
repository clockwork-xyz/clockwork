use clap::{App, Arg};

pub fn app() -> App<'static> {
    App::new("admin")
        .about("Run admin instructions against Cronos")
        .subcommand(cancel_task_app())
}

fn cancel_task_app() -> App<'static> {
    App::new("cancel").about("Cancels a scheduled task").arg(
        Arg::new("address")
            .index(1)
            .takes_value(true)
            .help("A task address"),
    )
}
