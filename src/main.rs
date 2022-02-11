use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches, SubCommand};

fn main() {
    let matches = App::new("Cronos")
        .bin_name("cronos")
        .about("Cronos is an instruction scheduler for Solana")
        .version("0.0.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(daemon_app())
        .get_matches();

    // do_main(&matches);
    println!("{:?}", matches.subcommand());

    println!("The app: {:?}", matches);
    println!("Hello, world!");
}

fn daemon_app() -> App<'static> {
    App::new("daemon")
        .about("Manage your daemon account")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("create").about("Create a daemon account"))
}
