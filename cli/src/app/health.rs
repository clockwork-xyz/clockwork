use clap::App;

pub fn app() -> App<'static> {
    App::new("health")
        .about("Check the Cronos health")
        .subcommand(App::new("reset").about("Reset the Cronos health tracker"))
}
