use crate::{cli::CliCommand, config::CliConfig, errors::CliError};
use anyhow::Result;
use clap::ArgMatches;
use clockwork_client::Client;
use solana_sdk::signature::read_keypair_file;

pub fn process(matches: &ArgMatches) -> Result<(), CliError> {
    // Parse command and config
    let command = CliCommand::try_from(matches)?;

    match command {
        // Set solana config if using localnet command
        CliCommand::Localnet { program_infos: _ } => {
            // TODO Verify the Solana CLI version is compatable with this build.
            set_solana_config().map_err(|err| CliError::FailedLocalnet(err.to_string()))?
        }
        _ => {}
    }

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
            epoch_queue,
            hasher_queue,
        } => super::config::set(&client, admin, epoch_queue, hasher_queue),
        CliCommand::Crontab { schedule } => super::crontab::get(&client, schedule),
        CliCommand::DelegationCreate { worker_id } => super::delegation::create(&client, worker_id),
        CliCommand::DelegationDeposit {
            amount,
            delegation_id,
            worker_id,
        } => super::delegation::deposit(&client, amount, delegation_id, worker_id),
        CliCommand::DelegationGet {
            delegation_id,
            worker_id,
        } => super::delegation::get(&client, delegation_id, worker_id),
        CliCommand::Initialize { mint } => super::initialize::initialize(&client, mint),
        CliCommand::Localnet { program_infos } => super::localnet::start(&client, program_infos),
        CliCommand::PoolGet { id } => super::pool::get(&client, id),
        CliCommand::PoolList {} => super::pool::list(&client),
        CliCommand::PoolUpdate { id, size } => super::pool::update(&client, id, size),
        CliCommand::QueueCreate {
            id,
            kickoff_instruction,
            trigger,
        } => super::queue::create(&client, id, kickoff_instruction, trigger),
        CliCommand::QueueDelete { id } => super::queue::delete(&client, id),
        CliCommand::QueueGet { id } => super::queue::get(&client, id),
        CliCommand::QueuePause { id } => super::queue::pause(&client, id),
        CliCommand::QueueResume { id } => super::queue::resume(&client, id),
        CliCommand::QueueStop { id } => super::queue::stop(&client, id),
        CliCommand::QueueUpdate {
            id,
            rate_limit,
            schedule,
        } => super::queue::update(&client, id, rate_limit, schedule),
        CliCommand::RegistryGet => super::registry::get(&client),
        CliCommand::RegistryUnlock => super::registry::unlock(&client),
        CliCommand::WebhookRequestNew {
            api,
            id,
            method,
            route,
        } => super::webhook::request_new(&client, api, id, method, route),
        CliCommand::WorkerCreate { signatory } => super::worker::create(&client, signatory, false),
        CliCommand::WorkerGet { id } => super::worker::get(&client, id),
    }
}

fn set_solana_config() -> Result<()> {
    let mut process = std::process::Command::new("solana")
        .args(&["config", "set", "--url", "l"])
        .spawn()
        .expect("Failed to set solana config");
    process.wait()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}
