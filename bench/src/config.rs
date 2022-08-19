use std::time::Duration;

use solana_sdk::commitment_config::CommitmentConfig;

pub const DEFAULT_RPC_TIMEOUT_SECONDS: Duration = Duration::from_secs(30);
pub const DEFAULT_CONFIRM_TX_TIMEOUT_SECONDS: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub struct CliConfig {
    pub json_rpc_url: String,
    pub websocket_url: String,
    pub keypair_path: String,
    pub rpc_timeout: Duration,
    pub commitment: CommitmentConfig,
    pub confirm_transaction_initial_timeout: Duration,
}

impl CliConfig {
    pub fn load() -> Self {
        let config_file = solana_cli_config::CONFIG_FILE.as_ref().unwrap().as_str();
        let solana_config = solana_cli_config::Config::load(config_file).unwrap();

        CliConfig {
            json_rpc_url: solana_config.json_rpc_url,
            websocket_url: solana_config.websocket_url,
            keypair_path: solana_config.keypair_path,
            rpc_timeout: DEFAULT_RPC_TIMEOUT_SECONDS,
            commitment: CommitmentConfig::confirmed(),
            confirm_transaction_initial_timeout: DEFAULT_CONFIRM_TX_TIMEOUT_SECONDS,
        }
    }
}
