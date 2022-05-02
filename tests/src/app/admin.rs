use clap::Command;

pub fn app() -> Command<'static> {
    Command::new("admin")
        .about("Run admin instructions against Cronos")
        .subcommand_required(true)
        .subcommand(admin_initialize_app())
}

fn admin_initialize_app() -> Command<'static> {
    Command::new("initialize").about("Initializes the Cronos program")
}
