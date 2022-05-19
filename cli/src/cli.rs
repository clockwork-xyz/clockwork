use clap::{Arg, Command};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    // Task commands
    TaskGet { address: Pubkey },

    // Admin commands
    Initialize { mint: Pubkey },

    // Node commands
    NodeRegister { delegate: Keypair },
    NodeStake { amount: u64, delegate: Pubkey },

    // Pool commands
    PoolGet,

    // Manager commands
    ManagerCreate,
    ManagerGet { address: Pubkey },

    // Queue commands
    QueueCreate { schedule: String },
    QueueGet { address: Pubkey },

    // Utility commands
    Clock,
    Config,
    Health,
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Account not found: {0}")]
    AccountNotFound(String),
    #[error("Account data could not be parsed: {0}")]
    AccountDataNotParsable(String),
    #[error("Bad client: {0}")]
    BadClient(String),
    #[error("Bad parameter: {0}")]
    BadParameter(String),
    #[error("Command not recognized: {0}")]
    CommandNotRecognized(String),
}

pub fn app() -> Command<'static> {
    Command::new("Cronos")
        .bin_name("cronos")
        .about("Automation infrastructure for Solana")
        .version(version!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("initialize")
                .about("Initialize the Cronos programs")
                .arg(
                    Arg::new("mint")
                        .long("mint")
                        .short('m')
                        .takes_value(true)
                        .required(true)
                        .help("Mint address of network token"),
                ),
        )
        .subcommand(
            Command::new("task")
                .about("Manage an task")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("get").about("Get an task").arg(
                        Arg::new("address")
                            .index(1)
                            .takes_value(true)
                            .required(true)
                            .help("Public address of a task"),
                    ),
                ),
        )
        .subcommand(
            Command::new("node")
                .about("Manage your nodes")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("register")
                        .about("Register a new node with the Cronos network")
                        .arg(
                            Arg::new("delegate")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("Filepath to the delegate wallet"),
                        ),
                )
                .subcommand(
                    Command::new("stake")
                        .about("Stake CRON with your Solana node")
                        .arg(
                            Arg::new("amount")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("The number of tokens to stake"),
                        )
                        .arg(
                            Arg::new("delegate")
                                .index(2)
                                .takes_value(true)
                                .required(true)
                                .help("The delegate address to stake tokens with"),
                        ),
                ),
        )
        .subcommand(Command::new("pool").about("Get the delegate pool info"))
        .subcommand(
            Command::new("manager")
                .about("Manage your managers")
                .arg_required_else_help(true)
                .subcommand(Command::new("create").about("Create a new manager"))
                .subcommand(
                    Command::new("get").about("Get a manager").arg(
                        Arg::new("address")
                            .index(1)
                            .takes_value(true)
                            .required(true)
                            .help("Public address of a manager"),
                    ),
                ),
        )
        .subcommand(
            Command::new("queue")
                .about("Manage your queues")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new queue")
                        .arg(
                            Arg::new("filepath")
                                .long("filepath")
                                .short('f')
                                .takes_value(true)
                                .required(true)
                                .help("Filepath to the instruction to invoke"),
                        )
                        .arg(
                            Arg::new("schedule")
                                .long("schedule")
                                .short('s')
                                .takes_value(true)
                                .required(false)
                                .help("Schedule to invoke the instruction"),
                        ),
                )
                .subcommand(
                    Command::new("get").about("Get a queue").arg(
                        Arg::new("address")
                            .index(1)
                            .takes_value(true)
                            .required(true)
                            .help("Public address of a queue"),
                    ),
                ),
        )
        .subcommand(Command::new("clock").about("Display the current Solana clock time"))
        .subcommand(Command::new("config").about("Manage the Cronos configs"))
        .subcommand(Command::new("health").about("Get the current system health"))
}
