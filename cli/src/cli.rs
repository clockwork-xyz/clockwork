use crate::parser::ProgramInfo;
use clap::{Arg, ArgGroup, Command};
use clockwork_client::webhook::objects::HttpMethod;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    // API commands
    ApiNew {
        ack_authority: Pubkey,
        base_url: String,
    },

    // Config commands
    ConfigGet,

    // Delegation
    DelegationCreate {
        worker_id: u64,
    },
    DelegationDeposit {
        amount: u64,
        delegation_id: u64,
        worker_id: u64,
    },
    DelegationGet {
        delegation_id: u64,
        worker_id: u64,
    },

    Initialize {
        mint: Pubkey,
    },

    // Localnet commands
    Localnet {
        program_infos: Vec<ProgramInfo>,
    },

    // Pool commands
    PoolGet {
        id: u64,
    },
    PoolList {},

    // Queue commands
    QueueGet {
        id: String,
    },
    QueueUpdate {
        id: String,
        rate_limit: Option<u64>,
    },

    // Registry
    RegistryGet,

    // Http
    WebhookRequestNew {
        api: Pubkey,
        id: String,
        method: HttpMethod,
        route: String,
    },

    // Worker commands
    WorkerCreate {
        signatory: Keypair,
    },
    WorkerGet {
        id: u64,
    },
}

pub fn app() -> Command<'static> {
    Command::new("Clockwork")
        .bin_name("clockwork")
        .about("An automation engine for the Solana blockchain")
        .version(version!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("config")
                .about("Manage the Clockwork network config")
                .arg_required_else_help(true)
                .subcommand(Command::new("get").about("Get a config value"))
                .subcommand(
                    Command::new("set")
                        .about("Set a config value")
                        .arg(
                            Arg::new("admin")
                                .long("admin")
                                .value_name("PUBKEY")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("grace_period")
                                .long("grace_period")
                                .value_name("NUM_SECONDS")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("spam_penalty")
                                .long("spam_penalty")
                                .value_name("NUM_LAMPORTS")
                                .takes_value(true),
                        )
                        .group(
                            ArgGroup::new("config_settings")
                                .args(&["admin", "worker_fee", "grace_period", "spam_penalty"])
                                .multiple(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("delegation")
                .about("Manage a stake delegation to a Clockwork worker")
                .subcommand(
                    Command::new("create")
                        .about("Create a new delegation")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("worker_id")
                                .index(1)
                                .takes_value(true)
                                .required(false)
                                .help("The ID of the worker to create a delegation with"),
                        ),
                )
                .subcommand(
                    Command::new("deposit")
                        .about("Deposit CLOCK to a delegation account")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("amount")
                                .index(1)
                                .takes_value(true)
                                .required(false)
                                .help("The number of tokens to deposit"),
                        )
                        .arg(
                            Arg::new("delegation_id")
                                .index(2)
                                .takes_value(true)
                                .required(false)
                                .help("The ID of the delegation to deposit into"),
                        )
                        .arg(
                            Arg::new("worker_id")
                                .index(3)
                                .takes_value(true)
                                .required(false)
                                .help("The ID of the worker"),
                        ),
                )
                .subcommand(
                    Command::new("get")
                        .about("Get a delegation")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("delegation_id")
                                .index(1)
                                .takes_value(true)
                                .required(false)
                                .help("The ID of the delegation"),
                        )
                        .arg(
                            Arg::new("worker_id")
                                .index(2)
                                .takes_value(true)
                                .required(false)
                                .help("The ID of the worker"),
                        ),
                ),
        )
        .subcommand(
            Command::new("initialize")
                .about("Initialize the Clockwork network program")
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
            Command::new("localnet")
                .about("Launch a local Clockwork worker for app development and testing")
                .arg(
                    Arg::with_name("bpf_program")
                        .long("bpf-program")
                        .value_names(&["ADDRESS_OR_KEYPAIR", "BPF_PROGRAM.SO"])
                        .takes_value(true)
                        .number_of_values(2)
                        .multiple(true)
                        .help(
                            "Add a BPF program to the genesis configuration. \
                       If the ledger already exists then this parameter is silently ignored. \
                       First argument can be a pubkey string or path to a keypair",
                        ),
                ),
        )
        .subcommand(
            Command::new("pool")
                .about("Manage the Clockwork network worker pools")
                .subcommand(
                    Command::new("get")
                        .about("Get a pool")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("id")
                                .index(1)
                                .takes_value(true)
                                .required(false)
                                .help("The ID of the pool to lookup"),
                        ),
                )
                .subcommand(Command::new("list").about("List the pools")),
        )
        .subcommand(
            Command::new("queue")
                .about("Manage your Clockwork transaction queues")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("get").about("Lookup the queue").arg(
                        Arg::new("id")
                            .index(1)
                            .takes_value(true)
                            .required(false)
                            .help("The id of the queue to lookup"),
                    ),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update a property of a queue")
                        .arg(
                            Arg::new("id")
                                .index(1)
                                .takes_value(true)
                                .required(false)
                                .help("The id of the queue to lookup"),
                        )
                        .arg(
                            Arg::new("rate_limit")
                                .long("rate_limit")
                                .short('r')
                                .takes_value(true)
                                .required(false)
                                .help(
                                    "The maximum number of cranks allowed per slot for this queue",
                                ),
                        ),
                ),
        )
        .subcommand(Command::new("registry").about("Lookup the Clockwork network registry"))
        .subcommand(Command::new("snapshot").about("Lookup the current Clockwork network registry"))
        .subcommand(
            Command::new("worker")
                .about("Manage your workers")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Register a new worker with the Clockwork network")
                        .arg(
                            Arg::new("signatory_keypair")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("Filepath to the worker's signatory keypair"),
                        ),
                )
                .subcommand(
                    Command::new("get")
                        .about("Lookup a worker on the Clockwork network")
                        .arg(
                            Arg::new("id")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("The ID of the worker to lookup"),
                        ),
                ),
        )
}
