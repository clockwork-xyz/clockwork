use clap::{Arg, Command};

pub fn app() -> Command<'static> {
    Command::new("bench")
        .about("Cronos testing bench")
        // .subcommand_required(true)
        .arg(
            Arg::new("count")
                .long("count")
                .short('c')
                .takes_value(true)
                .help("The number of tasks to be ran in a single test (e.g. -c 10000)"),
        )
        .arg(
            Arg::new("time")
                .long("time")
                .short('t')
                .takes_value(true)
                .help("The time for how long a test should run in seconds (e.g. -t 30). Should be no greater than 59"),
        )
        .arg(
            Arg::new("percent")
                .long("percent")
                .short('p')
                .takes_value(true)
                .help("The percentage of tasks to executed in parallel 0-1 (e.g. -p .9)"),
        )
}
