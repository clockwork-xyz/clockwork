use log::info;
use solana_client::tpu_client::{TpuClient, TpuClientConfig, DEFAULT_FANOUT_SLOTS};
use std::{
    fmt::Debug,
    fs::File,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use solana_client::{client_error, rpc_client::RpcClient};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    program_error::ProgramError,
    pubkey::Pubkey,
    signature::{read_keypair, Keypair, Signature, Signer},
    signers::Signers,
    transaction::Transaction,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    Client(#[from] client_error::ClientError),

    #[error(transparent)]
    Program(#[from] ProgramError),
}

pub struct Client {
    pub client: TpuClient,
    pub payer: Keypair,
}

impl Client {
    pub fn new(keypath: String, rpc_url: String, websocket_url: String) -> Self {
        let payer = read_keypair(&mut File::open(keypath).unwrap()).unwrap();
        let rpc_client = Arc::new(RpcClient::new_with_commitment::<String>(
            rpc_url,
            CommitmentConfig::confirmed(),
        ));
        let client = TpuClient::new(
            rpc_client,
            &websocket_url,
            TpuClientConfig {
                fanout_slots: DEFAULT_FANOUT_SLOTS,
            },
        )
        .unwrap();
        Self { client, payer }
    }

    pub fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }

    pub fn send<T: Signers>(
        &self,
        ixs: &[Instruction],
        signers: &T,
    ) -> Result<Signature, ClientError> {
        let mut tx = Transaction::new_with_payer(ixs, Some(&self.payer_pubkey()));
        tx.sign(signers, self.rpc_client().get_latest_blockhash()?);
        let b = self.send_transaction(&tx);
        info!("Submitted tx: {} {}", tx.signatures[0], b);
        Ok(tx.signatures[0])
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tpu client payer {}", self.payer_pubkey())
    }
}

impl Deref for Client {
    type Target = TpuClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}
