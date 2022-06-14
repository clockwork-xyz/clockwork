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

    #[error("Failed to create the tpu client")]
    BadTpuClient,
}

pub type ClientResult<T> = Result<T, ClientError>;

pub struct Client {
    pub client: RpcClient,
    pub payer: Keypair,
}

impl Client {
    pub fn new(keypath: String, url: String) -> Self {
        let payer = read_keypair(&mut File::open(keypath).unwrap()).unwrap();
        let client = RpcClient::new_with_commitment::<String>(url, CommitmentConfig::confirmed());
        Self { client, payer }
    }

    pub fn get<T: AccountDeserialize>(&self, pubkey: &Pubkey) -> ClientResult<T> {
        let data = self.client.get_account_data(pubkey)?;
        Ok(T::try_deserialize(&mut data.as_slice())
            .map_err(|_| ClientError::DeserializationError)?)
    }

    pub fn get_clock(&self) -> ClientResult<Clock> {
        let clock_pubkey = Pubkey::from_str("SysvarC1ock11111111111111111111111111111111").unwrap();
        let clock_data = self.client.get_account_data(&clock_pubkey)?;
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
        Ok(self.client.get_latest_blockhash()?)
    }

    pub fn airdrop(&self, to_pubkey: &Pubkey, lamports: u64) -> ClientResult<Signature> {
        let blockhash = self.client.get_latest_blockhash()?;
        let signature = self.request_airdrop_with_blockhash(to_pubkey, lamports, &blockhash)?;
        self.confirm_transaction_with_spinner(&signature, &blockhash, self.commitment())?;
        Ok(signature)
    }

    pub fn send<T: Signers>(&self, ixs: &[Instruction], signers: &T) -> ClientResult<Signature> {
        let mut tx = Transaction::new_with_payer(ixs, Some(&self.payer_pubkey()));
        tx.sign(signers, self.latest_blockhash()?);
        Ok(self.send_transaction(&tx)?)
    }

    pub fn send_via_tpu<T: Signers>(
        &self,
        ixs: &[Instruction],
        signers: &T,
    ) -> ClientResult<Signature> {
        let mut tx = Transaction::new_with_payer(ixs, Some(&self.payer_pubkey()));
        tx.sign(signers, self.latest_blockhash()?);

        // let c = Arc::new(self.client);
        let c = Arc::new(RpcClient::new_with_commitment::<String>(
            self.client.url(),
            CommitmentConfig::confirmed(),
        ));
        let tpu_client = TpuClient::new(
            c,
            "ws://root@145.40.64.193:8899",
            TpuClientConfig {
                fanout_slots: DEFAULT_FANOUT_SLOTS,
            },
        )
        .map_err(|_err| ClientError::BadTpuClient)?;

        tpu_client.send_transaction(&tx);

        Ok(tx.signatures[0])
    }

    pub fn send_with_config<T: Signers>(
        &self,
        ixs: &[Instruction],
        signers: &T,
        config: RpcSendTransactionConfig,
    ) -> ClientResult<Signature> {
        let mut tx = Transaction::new_with_payer(ixs, Some(&self.payer_pubkey()));
        tx.sign(signers, self.latest_blockhash()?);
        Ok(self.client.send_transaction_with_config(&tx, config)?)
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
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RPC client payer {}", self.payer_pubkey())
    }
}

impl Deref for Client {
    type Target = RpcClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}
