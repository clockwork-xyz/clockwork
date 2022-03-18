use clap::Command;

pub fn app() -> Command<'static> {
    Command::new("clock").about("Check the current Solana clock time")
}
