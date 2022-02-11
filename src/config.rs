use std::{sync::Arc, time::Duration};

use clap::ArgMatches;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_client_helpers::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};

use crate::{command::CliCommand, error::CliError};

pub const DEFAULT_RPC_TIMEOUT_SECONDS: Duration = Duration::from_secs(30);
pub const DEFAULT_CONFIRM_TX_TIMEOUT_SECONDS: Duration = Duration::from_secs(5);

pub struct CliConfig<'a> {
    pub command: CliCommand,
    pub json_rpc_url: String,
    pub websocket_url: String,
    pub signers: Vec<&'a dyn Signer>,
    pub keypair_path: String,
    pub rpc_client: Option<Arc<RpcClient>>,
    pub rpc_timeout: Duration,
    pub verbose: bool,
    pub commitment: CommitmentConfig,
    pub send_transaction_config: RpcSendTransactionConfig,
    pub confirm_transaction_initial_timeout: Duration,
}

impl Default for CliConfig<'_> {
    fn default() -> Self {
        CliConfig {
            command: CliCommand::Health {},
            json_rpc_url: solana_cli_config::Config::default().json_rpc_url,
            websocket_url: solana_cli_config::Config::default().websocket_url,
            signers: Vec::new(),
            keypair_path: solana_cli_config::Config::default().keypair_path,
            rpc_client: None,
            rpc_timeout: DEFAULT_RPC_TIMEOUT_SECONDS,
            verbose: false,
            commitment: CommitmentConfig::confirmed(),
            send_transaction_config: RpcSendTransactionConfig::default(),
            confirm_transaction_initial_timeout: DEFAULT_CONFIRM_TX_TIMEOUT_SECONDS,
        }
    }
}

impl TryFrom<&ArgMatches> for CliConfig<'_> {
    type Error = CliError;

    fn try_from(_matches: &ArgMatches) -> Result<Self, Self::Error> {
        // TODO parse config arguments
        Ok(CliConfig::default())
    }
}
