use clap::App;

pub fn app() -> App<'static> {
    App::new("daemon")
        .about("Manage your daemon")
        .subcommand(App::new("new").about("Create a daemon account"))
}
