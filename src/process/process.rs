use std::sync::Arc;

use solana_client_helpers::{Client, RpcClient};

use crate::{command::CliCommand, config::CliConfig, error::CliError, signer::load_keypair};

pub fn process_command(command: &CliCommand, config: &CliConfig) -> Result<(), CliError> {
    let payer = load_keypair(&config);
    let client = RpcClient::new_with_timeouts_and_commitment(
        config.json_rpc_url.to_string(),
        config.rpc_timeout,
        config.commitment,
        config.confirm_transaction_initial_timeout,
    );
    let client = Arc::new(Client { client, payer });

    match command {
        &CliCommand::DaemonNew => super::daemon_new::process(&client),
        &CliCommand::DaemonData => super::daemon_data::process(&client),
        &CliCommand::HealthCheck => super::health_check::process(&client),
        &CliCommand::TaskData { address } => super::task_data::process(&client, &address),
        _ => Err(CliError::CommandNotImplemented(command.to_string())),
    }
}
