use clap::{App, Arg};

pub fn app() -> App<'static> {
    App::new("task")
        .about("Manage your tasks")
        .subcommand(task_cancel_app())
        .subcommand(task_new_app())
        .arg(
            Arg::new("address")
                .index(1)
                .takes_value(true)
                .help("A task address"),
        )
}

fn task_cancel_app() -> App<'static> {
    App::new("cancel").about("Cancels a task").arg(
        Arg::new("address")
            .index(1)
            .takes_value(true)
            .help("A task address"),
    )
}

fn task_new_app() -> App<'static> {
    App::new("new")
        .about("Creates a new task")
        .arg(
            Arg::new("filepath")
                .long("filepath")
                .short('f')
                .required(true)
                .takes_value(true)
                .help("Filepath to the instruction to invoke"),
        )
        .arg(
            Arg::new("exec_at")
                .long("exec_at")
                .short('e')
                .takes_value(true)
                .required(false)
                .help("When to invoke the instruction"),
        )
        .arg(
            Arg::new("stop_at")
                .long("stop_at")
                .short('s')
                .takes_value(true)
                .required(false)
                .help("When to stop invoke the instruction"),
        )
        .arg(
            Arg::new("recurr")
                .long("recurr")
                .short('r')
                .takes_value(true)
                .required(false)
                .help("The duration to wait between instruction invocations"),
        )
}
