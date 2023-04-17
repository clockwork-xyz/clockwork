use anchor_lang::{
    prelude::{AccountMeta, Clock, Pubkey},
    solana_program::{instruction::Instruction, sysvar},
    Discriminator, InstructionData, ToAccountMetas,
};
use clockwork_thread_program_v2::state::{Thread, Trigger, VersionedThread, PAYER_PUBKEY};
use js_sys::WebAssembly::RuntimeError;
use serde::{Deserialize, Serialize};
use solana_client_wasm::{
    solana_sdk::{
        account::Account,
        commitment_config::CommitmentConfig,
        compute_budget::ComputeBudgetInstruction,
        transaction::{Transaction, TransactionError}, signature::Signature,
    },
    utils::{
        rpc_config::{
            GetConfirmedSignaturesForAddress2Config, RpcAccountInfoConfig, RpcBlockConfig,
            RpcProgramAccountsConfig, RpcTransactionConfig,
        },
        rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
        rpc_response::RpcConfirmedTransactionStatusWithSignature,
    },
    ClientResult, WasmClient,
};
use solana_extra_wasm::{account_decoder::UiAccountEncoding, transaction_status::{UiConfirmedBlock, EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding}};
use std::str::FromStr;

pub struct Client {
    pub client: WasmClient,
    pub cluster: Cluster,
}

impl Client {
    pub fn new() -> Self {
        Self::new_with_config(Cluster::Mainnet)
    }

    pub fn new_with_config(cluster: Cluster) -> Self {
        let rpc_url = cluster.url();
        let client = WasmClient::new(&rpc_url);

        Self { client, cluster }
    }

    pub async fn get_account(&self, address: Pubkey) -> ClientResult<Option<Account>> {
        self.client
            .get_account_with_config(
                &address,
                RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    data_slice: None,
                    commitment: Some(CommitmentConfig::finalized()),
                    min_context_slot: None,
                },
            )
            .await
    }

    pub async fn get_clock(&self) -> ClientResult<Clock> {
        self.client
            .get_account_with_config(
                &sysvar::clock::ID,
                RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    data_slice: None,
                    commitment: Some(CommitmentConfig::finalized()),
                    min_context_slot: None,
                },
            )
            .await
            .map(|account| bincode::deserialize::<Clock>(account.unwrap().data.as_slice()).unwrap())
    }

    pub async fn get_account_transaction(&self, signature:  &Signature) -> ClientResult<EncodedConfirmedTransactionWithStatusMeta> {
        self.client.get_transaction_with_config(
            signature, 
            RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Base64),
                commitment: Some(CommitmentConfig::finalized()),
                max_supported_transaction_version: None,
            },
        )
        .await
    }

    pub async fn get_threads(&self) -> ClientResult<Vec<(VersionedThread, Account)>> {
        let accounts = self
            .client
            .get_program_accounts_with_config(
                &clockwork_thread_program_v2::ID,
                RpcProgramAccountsConfig {
                    filters: Some(vec![RpcFilterType::Memcmp(Memcmp {
                        offset: 0,
                        bytes: MemcmpEncodedBytes::Bytes(Thread::discriminator().to_vec()),
                        encoding: None,
                    })]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: None,
                        commitment: Some(CommitmentConfig::finalized()),
                        min_context_slot: None,
                    },
                    with_context: None,
                },
            )
            .await
            .unwrap()
            .iter()
            .map(|acc| {
                (
                    VersionedThread::try_from(acc.1.data.clone()).unwrap(),
                    acc.1.clone(),
                )
            })
            .collect::<Vec<(VersionedThread, Account)>>()
            .to_vec();

        Ok(accounts)
    }

    pub async fn get_transaction_history(
        &self,
        address: Pubkey,
    ) -> ClientResult<Vec<RpcConfirmedTransactionStatusWithSignature>> {
        self.client
            .get_signatures_for_address_with_config(
                &address,
                GetConfirmedSignaturesForAddress2Config {
                    before: None,
                    until: None,
                    limit: Some(10),
                    commitment: Some(CommitmentConfig::confirmed()),
                },
            )
            .await
    }

    pub async fn get_thread(&self, pubkey: Pubkey) -> ClientResult<(VersionedThread, Account)> {
        let account = self
            .client
            .get_account_with_config(
                &pubkey,
                RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    data_slice: None,
                    commitment: Some(CommitmentConfig::finalized()),
                    min_context_slot: None,
                },
            )
            .await
            .unwrap()
            .unwrap();

        Ok((
            VersionedThread::try_from(account.clone().data).unwrap(),
            account,
        ))
    }

    pub async fn simulate_thread(
        &self,
        thread: VersionedThread,
        thread_pubkey: Pubkey,
    ) -> ClientResult<(Option<TransactionError>, Option<Vec<String>>)> {
        let signatory_pubkey =
            Pubkey::from_str("GuJVu6wky7zeVaPkGaasC5vx1eVoiySbEv7UFKZAu837").unwrap();
        let worker_pubkey =
            Pubkey::from_str("EvoeDp2WL1TFdLdf9bfJaznsf3YVByisvHM5juYdFBuq").unwrap();

        let first_instruction = if thread.next_instruction().is_some() {
            build_exec_ix(
                thread.clone(),
                thread_pubkey,
                signatory_pubkey,
                worker_pubkey,
            )
        } else {
            build_kickoff_ix(
                thread.clone(),
                thread_pubkey,
                signatory_pubkey,
                worker_pubkey,
            )
        };

        let ixs: Vec<Instruction> = vec![
            ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
            first_instruction,
        ];

        let tx = Transaction::new_with_payer(&ixs, Some(&signatory_pubkey));

        // simulate transaction
        let sim_result = self.client.simulate_transaction(&tx).await.unwrap();
        Ok((sim_result.err, sim_result.logs))
    }

    pub async fn get_block(&self) -> Option<UiConfirmedBlock> {
        let slot = self
            .client
            .get_latest_blockhash_with_commitment(CommitmentConfig::processed())
            .await
            .unwrap()
            .1;
        self.client
            .get_block_with_config(
                slot,
                RpcBlockConfig {
                    encoding: None,
                    transaction_details: Some(
                        solana_extra_wasm::transaction_status::TransactionDetails::Signatures,
                    ),
                    rewards: Some(true),
                    commitment: Some(CommitmentConfig::processed()),
                    max_supported_transaction_version: None,
                },
            )
            .await
            .ok()
    }

    pub async fn get_transactions(
        &self,
        address: Pubkey,
    ) -> Vec<RpcConfirmedTransactionStatusWithSignature> {
        self.client
            .get_signatures_for_address_with_config(
                &address,
                GetConfirmedSignaturesForAddress2Config {
                    before: None,
                    until: None,
                    limit: Some(10),
                    commitment: Some(CommitmentConfig::processed()),
                },
            )
            .await
            .unwrap()
    }
}

fn build_kickoff_ix(
    thread: VersionedThread,
    thread_pubkey: Pubkey,
    signatory_pubkey: Pubkey,
    worker_pubkey: Pubkey,
) -> Instruction {
    // Build the instruction.
    let mut kickoff_ix = match thread {
        VersionedThread::V1(_) => Instruction {
            program_id: thread.program_id(),
            accounts: clockwork_thread_program_v1::accounts::ThreadKickoff {
                signatory: signatory_pubkey,
                thread: thread_pubkey,
                worker: worker_pubkey,
            }
            .to_account_metas(Some(false)),
            data: clockwork_thread_program_v1::instruction::ThreadKickoff {}.data(),
        },
        VersionedThread::V2(_) => Instruction {
            program_id: thread.program_id(),
            accounts: clockwork_thread_program_v2::accounts::ThreadKickoff {
                signatory: signatory_pubkey,
                thread: thread_pubkey,
                worker: worker_pubkey,
            }
            .to_account_metas(Some(false)),
            data: clockwork_thread_program_v2::instruction::ThreadKickoff {}.data(),
        },
    };

    // If the thread's trigger is account-based, inject the triggering account.
    match thread.trigger() {
        Trigger::Account {
            address,
            offset: _,
            size: _,
        } => kickoff_ix.accounts.push(AccountMeta {
            pubkey: address,
            is_signer: false,
            is_writable: false,
        }),
        _ => {}
    }

    kickoff_ix
}

fn build_exec_ix(
    thread: VersionedThread,
    thread_pubkey: Pubkey,
    signatory_pubkey: Pubkey,
    worker_pubkey: Pubkey,
) -> Instruction {
    // Build the instruction.
    let mut exec_ix = match thread {
        VersionedThread::V1(_) => Instruction {
            program_id: thread.program_id(),
            accounts: clockwork_thread_program_v1::accounts::ThreadExec {
                fee: clockwork_network_program::state::Fee::pubkey(worker_pubkey),
                penalty: clockwork_network_program::state::Penalty::pubkey(worker_pubkey),
                pool: clockwork_network_program::state::Pool::pubkey(0),
                signatory: signatory_pubkey,
                thread: thread_pubkey,
                worker: worker_pubkey,
            }
            .to_account_metas(Some(true)),
            data: clockwork_thread_program_v1::instruction::ThreadExec {}.data(),
        },
        VersionedThread::V2(_) => Instruction {
            program_id: thread.program_id(),
            accounts: clockwork_thread_program_v2::accounts::ThreadExec {
                fee: clockwork_network_program::state::Fee::pubkey(worker_pubkey),
                pool: clockwork_network_program::state::Pool::pubkey(0),
                signatory: signatory_pubkey,
                thread: thread_pubkey,
                worker: worker_pubkey,
            }
            .to_account_metas(Some(true)),
            data: clockwork_thread_program_v2::instruction::ThreadExec {}.data(),
        },
    };

    if let Some(next_instruction) = thread.next_instruction() {
        // Inject the target program account.
        exec_ix.accounts.push(AccountMeta::new_readonly(
            next_instruction.program_id,
            false,
        ));

        // Inject the worker pubkey as the dynamic "payer" account.
        for acc in next_instruction.clone().accounts {
            let acc_pubkey = if acc.pubkey == PAYER_PUBKEY {
                signatory_pubkey
            } else {
                acc.pubkey
            };
            exec_ix.accounts.push(match acc.is_writable {
                true => AccountMeta::new(acc_pubkey, false),
                false => AccountMeta::new_readonly(acc_pubkey, false),
            })
        }
    }

    exec_ix
}
#[derive(Copy, Clone, Default, Serialize, Deserialize)]
pub enum Cluster {
    #[default]
    Mainnet,
    Devnet,
    // Custom(String),
}

impl Cluster {
    fn url(&self) -> String {
        match self {
            Self::Mainnet => {
                "https://rpc.helius.xyz/?api-key=cafb5acc-3dc2-47a0-8505-77ea5ebc7ec6".to_string()
            }
            Self::Devnet => {
                "https://rpc-devnet.helius.xyz/?api-key=8f29b4e9-37a6-4775-88c6-6f971fe180ca"
                    .to_string()
            }
            // Self::Custom(rpc_url) => rpc_url.to_string(),
        }
    }
}

impl ToString for Cluster {
    fn to_string(&self) -> String {
        match self {
            Self::Mainnet => "Mainnet".to_string(),
            Self::Devnet => "Devnet".to_string(),
            // Self::Custom(rpc_url) => String::from(format!("Custom: {}", rpc_url)),
        }
    }
}

impl FromStr for Cluster {
    type Err = RuntimeError;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        match expression {
            "Mainnet" => Ok(Self::Mainnet),
            "Devnet" => Ok(Self::Devnet),
            // "Custom" => Ok(Self::Custom("".to_string())),
            _ => Err(RuntimeError::new("Invalid expression")),
        }
    }
}
