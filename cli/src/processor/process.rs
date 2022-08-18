use crate::{cli::CliCommand, config::CliConfig, errors::CliError};
use clap::ArgMatches;
use clockwork_client::Client;
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
        CliCommand::ConfigGet => super::config::get(&client),
        CliCommand::ConfigSet {
            admin,
            automation_fee,
        } => super::config::set(&client, admin, automation_fee),
        CliCommand::HttpRequestNew {
            api,
            id,
            method,
            route,
        } => super::http::request_new(&client, api, id, method, route),
        CliCommand::Initialize { mint } => super::initialize::initialize(&client, mint),
        CliCommand::Localnet {} => super::localnet::start(&client),
        CliCommand::NodeRegister { worker } => super::node::register(&client, worker),
        CliCommand::NodeStake { address, amount } => super::node::stake(&client, address, amount),
        CliCommand::PoolGet => super::pool::get(&client),
        CliCommand::QueueGet { address } => super::queue::get(&client, &address),
        CliCommand::RegistryGet => super::registry::get(&client),
        CliCommand::SnapshotGet { entry_id } => super::snapshot::get(&client, entry_id),
    }
}
