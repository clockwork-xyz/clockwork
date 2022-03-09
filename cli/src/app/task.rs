use clap::{Arg, Command};

pub fn app() -> Command<'static> {
    Command::new("task")
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

fn task_cancel_app() -> Command<'static> {
    Command::new("cancel").about("Cancels a task").arg(
        Arg::new("address")
            .index(1)
            .takes_value(true)
            .help("A task address"),
    )
}

fn task_new_app() -> Command<'static> {
    Command::new("new")
        .about("Creates a new task")
        .arg(
            Arg::new("filepath")
                .long("filepath")
                .short('f')
                .takes_value(true)
                .required(true)
                .help("Filepath to the instruction to invoke"),
        )
        .arg(
            Arg::new("schedule")
                .long("schedule")
                .short('s')
                .takes_value(true)
                .required(false)
                .help("Schedule to invoke the instruction"),
        )
}
