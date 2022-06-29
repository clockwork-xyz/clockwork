use solana_client::tpu_client::TpuSenderError;

use {
    solana_client::{
        client_error,
        rpc_client::RpcClient,
        tpu_client::{TpuClient as SolanaTpuClient, TpuClientConfig, DEFAULT_FANOUT_SLOTS},
    },
    solana_sdk::{
        commitment_config::CommitmentConfig,
        program_error::ProgramError,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    },
    std::{
        fmt::Debug,
        ops::{Deref, DerefMut},
        sync::Arc,
    },
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum TpuClientError {
    #[error(transparent)]
    Client(#[from] client_error::ClientError),

    #[error(transparent)]
    Program(#[from] ProgramError),
}

pub struct TpuClient {
    pub client: SolanaTpuClient,
    pub payer: Keypair,
}

impl TpuClient {
    pub fn new(
        payer: Keypair,
        rpc_url: String,
        websocket_url: String,
    ) -> Result<Self, TpuSenderError> {
        let rpc_client = Arc::new(RpcClient::new_with_commitment::<String>(
            rpc_url,
            CommitmentConfig::confirmed(),
        ));
        let client = SolanaTpuClient::new(
            rpc_client,
            &websocket_url,
            TpuClientConfig {
                fanout_slots: DEFAULT_FANOUT_SLOTS,
            },
        )?;
        Ok(Self { client, payer })
    }

    pub fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }
}

impl Debug for TpuClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tpu client payer {}", self.payer_pubkey())
    }
}

impl Deref for TpuClient {
    type Target = SolanaTpuClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for TpuClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}
