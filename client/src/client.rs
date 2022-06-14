use anchor_lang::{prelude::Clock, AccountDeserialize};
use solana_client::{
    rpc_config::RpcSendTransactionConfig,
    tpu_client::{TpuClient, TpuClientConfig, DEFAULT_FANOUT_SLOTS},
};
use std::{
    fmt::Debug,
    fs::File,
    ops::{Deref, DerefMut},
    str::FromStr,
    sync::Arc,
};

use solana_client::{client_error, rpc_client::RpcClient};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    hash::Hash,
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

    #[error("Failed to deserialize account data")]
    DeserializationError,
}

pub type ClientResult<T> = Result<T, ClientError>;

pub struct Client {
    pub rpc_client: Arc<RpcClient>,
    pub tpu_client: Arc<TpuClient>,
    pub payer: Keypair,
}

impl Client {
    pub fn new(keypath: String, rpc_url: String, websocket_url: String) -> Self {
        let payer = read_keypair(&mut File::open(keypath).unwrap()).unwrap();
        let rpc_client = Arc::new(RpcClient::new_with_commitment::<String>(
            rpc_url,
            CommitmentConfig::confirmed(),
        ));
        let tpu_client = Arc::new(
            TpuClient::new(
                rpc_client.clone(),
                &websocket_url,
                TpuClientConfig {
                    fanout_slots: DEFAULT_FANOUT_SLOTS,
                },
            )
            .unwrap(),
        );
        Self {
            rpc_client,
            tpu_client,
            payer,
        }
    }

    pub fn get<T: AccountDeserialize>(&self, pubkey: &Pubkey) -> ClientResult<T> {
        let data = self.rpc_client.get_account_data(pubkey)?;
        Ok(T::try_deserialize(&mut data.as_slice())
            .map_err(|_| ClientError::DeserializationError)?)
    }

    pub fn get_clock(&self) -> ClientResult<Clock> {
        let clock_pubkey = Pubkey::from_str("SysvarC1ock11111111111111111111111111111111").unwrap();
        let clock_data = self.rpc_client.get_account_data(&clock_pubkey)?;
        Ok(bincode::deserialize::<Clock>(&clock_data)
            .map_err(|_| ClientError::DeserializationError)?)
    }

    pub fn payer(&self) -> &Keypair {
        &self.payer
    }

    pub fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }

    pub fn latest_blockhash(&self) -> ClientResult<Hash> {
        Ok(self.rpc_client.get_latest_blockhash()?)
    }

    pub fn airdrop(&self, to_pubkey: &Pubkey, lamports: u64) -> ClientResult<Signature> {
        let blockhash = self.rpc_client.get_latest_blockhash()?;
        let signature = self.request_airdrop_with_blockhash(to_pubkey, lamports, &blockhash)?;
        self.confirm_transaction_with_spinner(&signature, &blockhash, self.commitment())?;
        Ok(signature)
    }

    pub fn send<T: Signers>(&self, ixs: &[Instruction], signers: &T) -> ClientResult<Signature> {
        let mut tx = Transaction::new_with_payer(ixs, Some(&self.payer_pubkey()));
        tx.sign(signers, self.latest_blockhash()?);
        Ok(self.send_transaction(&tx)?)
    }

    pub fn send_with_config<T: Signers>(
        &self,
        ixs: &[Instruction],
        signers: &T,
        config: RpcSendTransactionConfig,
    ) -> ClientResult<Signature> {
        let mut tx = Transaction::new_with_payer(ixs, Some(&self.payer_pubkey()));
        tx.sign(signers, self.latest_blockhash()?);
        Ok(self.rpc_client.send_transaction_with_config(&tx, config)?)
    }

    pub fn send_and_confirm<T: Signers>(
        &self,
        ixs: &[Instruction],
        signers: &T,
    ) -> ClientResult<Signature> {
        let mut tx = Transaction::new_with_payer(ixs, Some(&self.payer_pubkey()));
        tx.sign(signers, self.latest_blockhash()?);
        Ok(self.send_and_confirm_transaction(&tx)?)
    }

    pub fn send_via_tpu<T: Signers>(
        &self,
        ixs: &[Instruction],
        signers: &T,
    ) -> ClientResult<Signature> {
        let mut tx = Transaction::new_with_payer(ixs, Some(&self.payer_pubkey()));
        tx.sign(signers, self.latest_blockhash()?);
        self.tpu_client.send_transaction(&tx);
        Ok(tx.signatures[0])
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RPC client payer {}", self.payer_pubkey())
    }
}

impl Deref for Client {
    type Target = Arc<RpcClient>;

    fn deref(&self) -> &Self::Target {
        &self.rpc_client
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rpc_client
    }
}
