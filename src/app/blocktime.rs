use clap::App;

pub fn app() -> App<'static> {
    App::new("blocktime").about("Check the current Solana blocktime")
}
