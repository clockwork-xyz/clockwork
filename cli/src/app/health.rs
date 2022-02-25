use clap::Command;

pub fn app() -> Command<'static> {
    Command::new("health")
        .about("Check the Cronos health")
        .subcommand(Command::new("reset").about("Reset the Cronos health tracker"))
}
