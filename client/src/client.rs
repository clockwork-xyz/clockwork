use anchor_lang::{prelude::Clock, AccountDeserialize};
use anchor_spl::token::{
    spl_token::{self, state::Account as TokenAccount},
    Mint,
};
use clockwork_utils::ProgramLogsDeserializable;
use solana_client::{
    client_error, rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig,
    rpc_response::RpcSimulateTransactionResult,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    hash::Hash,
    instruction::Instruction,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    signers::Signers,
    system_instruction,
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
        let tx = self.transaction(ixs, signers)?;
        Ok(self.client.send_transaction_with_config(&tx, config)?)
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

pub trait SplToken {
    fn create_token_mint(&self, owner: &Pubkey, decimals: u8) -> ClientResult<Keypair>;
    fn create_token_account(&self, owner: &Pubkey, token_mint: &Pubkey) -> ClientResult<Keypair>;
    fn create_token_account_with_lamports(
        &self,
        owner: &Pubkey,
        token_mint: &Pubkey,
        lamports: u64,
    ) -> ClientResult<Keypair>;
    fn mint_to(
        &self,
        owner: &Keypair,
        token_mint: &Pubkey,
        account: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> ClientResult<()>;
    fn transfer_to(
        &self,
        owner: &Keypair,
        token_mint: &Pubkey,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> ClientResult<()>;
    fn get_associated_token_address(wallet_address: &Pubkey, token_mint: &Pubkey) -> Pubkey;
    fn create_associated_token_account(
        &self,
        funder: &Keypair,
        recipient: &Pubkey,
        token_mint: &Pubkey,
    ) -> ClientResult<Pubkey>;
    fn create_associated_token_account_by_payer(
        &self,
        recipient: &Pubkey,
        token_mint: &Pubkey,
    ) -> ClientResult<Pubkey>;
    fn close_token_account(
        &self,
        owner: &Keypair,
        account: &Pubkey,
        destination: &Pubkey,
    ) -> ClientResult<()>;
}

impl SplToken for Client {
    fn create_token_mint(&self, owner: &Pubkey, decimals: u8) -> ClientResult<Keypair> {
        let token_mint = Keypair::new();

        let mut transaction = Transaction::new_with_payer(
            &[
                system_instruction::create_account(
                    &self.payer_pubkey(),
                    &token_mint.pubkey(),
                    self.get_minimum_balance_for_rent_exemption(Mint::LEN)?,
                    Mint::LEN as u64,
                    &spl_token::id(),
                ),
                spl_token::instruction::initialize_mint(
                    &spl_token::id(),
                    &token_mint.pubkey(),
                    owner,
                    None,
                    decimals,
                )?,
            ],
            Some(&self.payer_pubkey()),
        );
        transaction.sign(&[self.payer(), &token_mint], self.latest_blockhash()?);
        self.send_and_confirm_transaction(&transaction)?;

        Ok(token_mint)
    }
    fn create_token_account(&self, owner: &Pubkey, token_mint: &Pubkey) -> ClientResult<Keypair> {
        self.create_token_account_with_lamports(
            owner,
            token_mint,
            self.get_minimum_balance_for_rent_exemption(TokenAccount::LEN)?,
        )
    }
    fn create_token_account_with_lamports(
        &self,
        owner: &Pubkey,
        token_mint: &Pubkey,
        lamports: u64,
    ) -> ClientResult<Keypair> {
        let token_account = Keypair::new();

        let mut transaction = Transaction::new_with_payer(
            &[
                system_instruction::create_account(
                    &self.payer_pubkey(),
                    &token_account.pubkey(),
                    lamports,
                    TokenAccount::LEN as u64,
                    &spl_token::id(),
                ),
                spl_token::instruction::initialize_account(
                    &spl_token::id(),
                    &token_account.pubkey(),
                    token_mint,
                    owner,
                )?,
            ],
            Some(&self.payer_pubkey()),
        );
        transaction.sign(&[self.payer(), &token_account], self.latest_blockhash()?);
        self.send_and_confirm_transaction(&transaction)?;

        Ok(token_account)
    }
    fn mint_to(
        &self,
        owner: &Keypair,
        token_mint: &Pubkey,
        account: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> ClientResult<()> {
        let mut transaction = Transaction::new_with_payer(
            &[spl_token::instruction::mint_to_checked(
                &spl_token::id(),
                token_mint,
                account,
                &owner.pubkey(),
                &[],
                amount,
                decimals,
            )?],
            Some(&self.payer_pubkey()),
        );
        transaction.sign(&[self.payer(), owner], self.latest_blockhash()?);
        self.send_and_confirm_transaction(&transaction)?;

        Ok(())
    }

    fn transfer_to(
        &self,
        authority: &Keypair,
        token_mint: &Pubkey,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> ClientResult<()> {
        let mut transaction = Transaction::new_with_payer(
            &[spl_token::instruction::transfer_checked(
                &spl_token::id(),
                source,
                token_mint,
                destination,
                &authority.pubkey(),
                &[],
                amount,
                decimals,
            )?],
            Some(&self.payer_pubkey()),
        );
        transaction.sign(&[self.payer(), authority], self.latest_blockhash()?);
        self.send_and_confirm_transaction(&transaction)?;

        Ok(())
    }
    fn get_associated_token_address(wallet_address: &Pubkey, token_mint: &Pubkey) -> Pubkey {
        spl_associated_token_account::get_associated_token_address(wallet_address, token_mint)
    }

    fn create_associated_token_account(
        &self,
        funder: &Keypair,
        recipient: &Pubkey,
        token_mint: &Pubkey,
    ) -> ClientResult<Pubkey> {
        let mut transaction = Transaction::new_with_payer(
            &[
                spl_associated_token_account::instruction::create_associated_token_account(
                    &funder.pubkey(),
                    recipient,
                    token_mint,
                    &anchor_spl::token::ID,
                ),
            ],
            Some(&self.payer_pubkey()),
        );
        if funder.pubkey() == self.payer_pubkey() {
            transaction.sign(&[self.payer()], self.latest_blockhash()?);
        } else {
            transaction.sign(&[self.payer(), funder], self.latest_blockhash()?);
        };
        self.send_and_confirm_transaction(&transaction)?;

        Ok(Self::get_associated_token_address(recipient, token_mint))
    }

    fn create_associated_token_account_by_payer(
        &self,
        recipient: &Pubkey,
        token_mint: &Pubkey,
    ) -> ClientResult<Pubkey> {
        self.create_associated_token_account(self.payer(), recipient, token_mint)
    }

    fn close_token_account(
        &self,
        owner: &Keypair,
        account: &Pubkey,
        destination: &Pubkey,
    ) -> ClientResult<()> {
        let mut transaction = Transaction::new_with_payer(
            &[spl_token::instruction::close_account(
                &spl_token::id(),
                account,
                destination,
                &owner.pubkey(),
                &[],
            )?],
            Some(&self.payer_pubkey()),
        );
        transaction.sign(&[self.payer(), owner], self.latest_blockhash()?);
        self.send_and_confirm_transaction(&transaction)?;

        Ok(())
    }
}
