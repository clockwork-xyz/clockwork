use crate::{
    cli::CliCommand, config::CliConfig, errors::CliError,
    processor::automation::parse_pubkey_from_id_or_address,
};
use anyhow::Result;
use clap::ArgMatches;
use clockwork_client::Client;
use solana_sdk::signature::read_keypair_file;

pub fn process(matches: &ArgMatches) -> Result<(), CliError> {
    // Parse command and config
    let command = CliCommand::try_from(matches)?;

    match command {
        // Set solana config if using localnet command
        CliCommand::Localnet {
            clone_addresses: _,
            network_url: _,
            program_infos: _,
        } => {
            // TODO Verify the Solana CLI version is compatable with this build.
            set_solana_config().map_err(|err| CliError::FailedLocalnet(err.to_string()))?
        }
        _ => {}
    }

    let config = CliConfig::load();

    // Build the RPC client
    let payer = read_keypair_file(&config.keypair_path)
        .map_err(|_| CliError::KeypairNotFound(config.keypair_path.clone()))?;

    let client = Client::new(payer, config.json_rpc_url.clone());

    // Process the command
    match command {
        CliCommand::ApiNew {
            ack_authority,
            base_url,
        } => super::api::api_new(&client, ack_authority, base_url),
        CliCommand::ConfigGet => super::config::get(&client),
        CliCommand::ConfigSet {
            admin,
            epoch_automation,
            hasher_automation,
        } => super::config::set(&client, admin, epoch_automation, hasher_automation),
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
        CliCommand::DelegationWithdraw {
            amount,
            delegation_id,
            worker_id,
        } => super::delegation::withdraw(&client, amount, delegation_id, worker_id),
        CliCommand::ExplorerGetAutomation { id, address } => {
            let pubkey = parse_pubkey_from_id_or_address(client.payer_pubkey(), id, address)?;
            super::explorer::automation_url(pubkey, config)
        }
        CliCommand::Initialize { mint } => super::initialize::initialize(&client, mint),
        CliCommand::Localnet {
            clone_addresses,
            network_url,
            program_infos,
        } => super::localnet::start(&client, clone_addresses, network_url, program_infos),
        CliCommand::PoolGet { id } => super::pool::get(&client, id),
        CliCommand::PoolList {} => super::pool::list(&client),
        CliCommand::PoolUpdate { id, size } => super::pool::update(&client, id, size),
        CliCommand::AutomationCrateInfo {} => super::automation::crate_info(&client),
        CliCommand::AutomationCreate {
            id,
            kickoff_instruction,
            trigger,
        } => super::automation::create(&client, id, vec![kickoff_instruction], trigger),
        CliCommand::AutomationDelete { id } => super::automation::delete(&client, id),
        CliCommand::AutomationPause { id } => super::automation::pause(&client, id),
        CliCommand::AutomationResume { id } => super::automation::resume(&client, id),
        CliCommand::AutomationReset { id } => super::automation::reset(&client, id),
        CliCommand::AutomationGet { id, address } => {
            let pubkey = parse_pubkey_from_id_or_address(client.payer_pubkey(), id, address)?;
            super::automation::get(&client, pubkey)
        }
        CliCommand::AutomationUpdate {
            id,
            rate_limit,
            schedule,
        } => super::automation::update(&client, id, rate_limit, schedule),
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
        CliCommand::WorkerUpdate { id, signatory } => super::worker::update(&client, id, signatory),
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
