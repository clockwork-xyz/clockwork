use clap::{App, AppSettings, Arg};

pub fn app() -> App<'static> {
    App::new("config")
        .about("Get Cronos program config info ")
        .subcommand(config_set_app())
}

fn config_set_app() -> App<'static> {
    App::new("set")
        .about("Set Cronos config variables")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(config_set_min_recurr_app())
    // .arg(
    //     Arg::new("name")
    //         .index(1)
    //         .takes_value(true)
    //         .help("The name of the config value to update"),
    // )
    // .arg(
    //     Arg::new("new_value")
    //         .index(1)
    //         .takes_value(true)
    //         .help("The value to update the config property to"),
    // )
}

fn config_set_min_recurr_app() -> App<'static> {
    App::new("min_recurr")
        .about("Update the config min recurr value")
        .arg(
            Arg::new("new_value")
                .index(1)
                .takes_value(true)
                .required(true)
                .help("The new minimum recurrence value to update the config to"),
        )
}
