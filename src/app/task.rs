use clap::{App, Arg};

pub fn app() -> App<'static> {
    App::new("task")
        .about("Manage your tasks")
        .subcommand(task_new_app())
        .arg(
            Arg::new("address")
                .index(1)
                .takes_value(true)
                // .required(true)
                .help("A task address"),
        )
}

fn task_new_app() -> App<'static> {
    App::new("new")
        .about("Schedule a new task")
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
}
