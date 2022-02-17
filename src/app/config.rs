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
        .subcommand(config_set_program_fee_app())
        .subcommand(config_set_worker_fee_app())
}

fn config_set_min_recurr_app() -> App<'static> {
    App::new("min_recurr")
        .about("Update the minimum recurrence interval")
        .arg(
            Arg::new("new_value")
                .index(1)
                .takes_value(true)
                .required(true)
                .help("The new minimum recurrence interval"),
        )
}

fn config_set_program_fee_app() -> App<'static> {
    App::new("program_fee").about("Update the program fee").arg(
        Arg::new("new_value")
            .index(1)
            .takes_value(true)
            .required(true)
            .help("The new program fee"),
    )
}

fn config_set_worker_fee_app() -> App<'static> {
    App::new("worker_fee").about("Update the worker fee").arg(
        Arg::new("new_value")
            .index(1)
            .takes_value(true)
            .required(true)
            .help("The new worker fee"),
    )
}
