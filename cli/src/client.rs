use anchor_lang::{prelude::Clock, AccountDeserialize};
use clockwork_utils::ProgramLogsDeserializable;
use solana_client::{
    client_error, rpc_client::RpcClient,
    rpc_response::RpcSimulateTransactionResult,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    hash::Hash,
    instruction::Instruction,
    program_error::ProgramError,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    signers::Signers,
    transaction::Transaction,
};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    str::FromStr,
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
    pub client: RpcClient,
    pub payer: Keypair,
}

impl Client {
    pub fn new(payer: Keypair, url: String) -> Self {
        let client = RpcClient::new_with_commitment::<String>(url, CommitmentConfig::processed());
        Self { client, payer }
    }

    pub fn get<T: AccountDeserialize>(&self, pubkey: &Pubkey) -> ClientResult<T> {
        let data = self.client.get_account_data(pubkey)?;
        T::try_deserialize(&mut data.as_slice()).map_err(|_| ClientError::DeserializationError)
    }

    pub fn get_clock(&self) -> ClientResult<Clock> {
        let clock_pubkey = Pubkey::from_str("SysvarC1ock11111111111111111111111111111111").unwrap();
        let clock_data = self.client.get_account_data(&clock_pubkey)?;
        bincode::deserialize::<Clock>(&clock_data).map_err(|_| ClientError::DeserializationError)
    }

    pub fn get_return_data<T: ProgramLogsDeserializable>(
        &self,
        ix: Instruction,
    ) -> ClientResult<T> {
        // let result = self.simulate_transaction(&[ix.clone()], &[self.payer()])?;

        // After we can upgrade our Solana SDK version to 1.14.0 we can just use the below code:
        // let data = result.logs;
        // Ok(T::try_from_slice(logs, &data)
        //     .map_err(|_| ClientError::DeserializationError)?)
        //
        // But for the time being since RpcSimulateTransactionResult.data does not exist yet,
        // We can only parse the logs ourselves to find the return_data
        let logs = self.get_instruction_logs(ix.clone())?;
        T::try_from_program_logs(logs, &ix.program_id)
            .map_err(|_| ClientError::DeserializationError)
    }

    pub fn get_instruction_logs(&self, ix: Instruction) -> ClientResult<Vec<String>> {
        let result = self.simulate_transaction(&[ix], &[self.payer()])?;
        let logs = result.logs.ok_or(ClientError::DeserializationError)?;
        Ok(logs)
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

    pub fn send_and_confirm<T: Signers>(
        &self,
        ixs: &[Instruction],
        signers: &T,
    ) -> ClientResult<Signature> {
        let tx = self.transaction(ixs, signers)?;
        Ok(self.send_and_confirm_transaction(&tx)?)
    }

    pub fn simulate_transaction<T: Signers>(
        &self,
        ixs: &[Instruction],
        signers: &T,
    ) -> ClientResult<RpcSimulateTransactionResult> {
        let tx = self.transaction(ixs, signers)?;
        let result = self.client.simulate_transaction(&tx)?;
        if result.value.err.is_some() {
            Err(ClientError::DeserializationError)
        } else {
            Ok(result.value)
        }
    }

    fn transaction<T: Signers>(
        &self,
        ixs: &[Instruction],
        signers: &T,
    ) -> ClientResult<Transaction> {
        let mut tx = Transaction::new_with_payer(ixs, Some(&self.payer_pubkey()));
        tx.sign(signers, self.latest_blockhash()?);
        Ok(tx)
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

