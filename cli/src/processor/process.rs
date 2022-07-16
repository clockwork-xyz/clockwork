use crate::{cli::CliCommand, config::CliConfig, errors::CliError};
use clap::ArgMatches;
use cronos_client::Client;
use solana_sdk::signature::read_keypair_file;

pub fn process(matches: &ArgMatches) -> Result<(), CliError> {
    // Parse command and config
    let command = CliCommand::try_from(matches)?;
    let config = CliConfig::load();

    // Build the RPC client
    let payer = read_keypair_file(config.keypair_path).unwrap();
    let client = Client::new(payer, config.json_rpc_url);

    // Process the command
    match command {
        CliCommand::ApiNew {
            ack_authority,
            base_url,
        } => super::api::api_new(&client, ack_authority, base_url),
        CliCommand::Clock => super::clock::get(&client),
        CliCommand::ConfigGet => super::config::get(&client),
        CliCommand::ConfigSet {
            admin,
            worker_fee,
            worker_holdout_period,
            worker_spam_penalty,
            program_fee,
        } => super::config::set(
            &client,
            admin,
            program_fee,
            worker_fee,
            worker_holdout_period,
            worker_spam_penalty,
        ),
        CliCommand::DelegateCreate => super::delegate::create(&client),
        CliCommand::DelegateGet { address } => super::delegate::get(&client, &address),
        CliCommand::Health => super::health::get(&client),
        CliCommand::HttpRequestNew {
            api,
            id,
            method,
            route,
        } => super::http::request_new(&client, api, id, method, route),
        CliCommand::Initialize { mint } => super::initialize::initialize(&client, mint),
        CliCommand::NodeGet { worker } => super::node::get_by_worker(&client, worker),
        CliCommand::NodeRegister { worker } => super::node::register(&client, worker),
        CliCommand::NodeStake { amount, worker } => super::node::stake(&client, amount, worker),
        CliCommand::PoolGet => super::pool::get(&client),
        CliCommand::QueueCreate { schedule } => super::queue::create(&client, schedule),
        CliCommand::QueueGet { address, task_id } => super::queue::get(&client, &address, task_id),
        CliCommand::RegistryGet => super::registry::get(&client),
        CliCommand::SnapshotGet { entry_id } => super::snapshot::get(&client, entry_id),
        CliCommand::TaskGet { address } => super::task::get(&client, &address),
    }
}
