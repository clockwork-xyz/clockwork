use clap::Command;

pub fn app() -> Command<'static> {
    Command::new("daemon")
        .about("Manage your daemon")
        .subcommand(Command::new("new").about("Create a daemon account"))
}
