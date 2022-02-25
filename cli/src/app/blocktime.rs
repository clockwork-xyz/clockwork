use clap::Command;

pub fn app() -> Command<'static> {
    Command::new("blocktime").about("Check the current Solana blocktime")
}
