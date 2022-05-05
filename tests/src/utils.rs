use std::{fs::File, sync::Arc};

use solana_clap_utils::keypair::DefaultSigner;
use solana_client_helpers::{Client, RpcClient};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    native_token::LAMPORTS_PER_SOL,
    signature::{read_keypair, Keypair},
    transaction::Transaction,
};

use crate::config::CliConfig;

pub fn new_client() -> Arc<Client> {
    let url = "http://localhost:8899";
    let client = Arc::new(Client {
        client: RpcClient::new_with_commitment(url, CommitmentConfig::confirmed()),
        payer: Keypair::new(),
    });
    client
        .airdrop(&client.payer_pubkey(), LAMPORTS_PER_SOL)
        .unwrap();
    client
}

pub fn _load_keypair(config: &CliConfig) -> Keypair {
    let signer = DefaultSigner::new("keypair".to_string(), &config.keypair_path);
    read_keypair(&mut File::open(signer.path.as_str()).unwrap()).unwrap()
}

pub fn sign_and_submit(client: &Arc<Client>, ixs: &[Instruction]) {
    let mut tx = Transaction::new_with_payer(ixs, Some(&client.payer_pubkey()));
    tx.sign(&vec![&client.payer], client.latest_blockhash().unwrap());
    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    println!(
        "Tx: {}",
        solana_explorer_url(SolanaExplorerAccountType::Tx, sig.to_string())
    );
}

pub enum SolanaExplorerAccountType {
    Tx,
}

pub fn solana_explorer_url(entity: SolanaExplorerAccountType, value: String) -> String {
    let base_url = "https://explorer.solana.com";
    let entity_str = match entity {
        SolanaExplorerAccountType::Tx => "tx",
    };
    format!("{}/{}/{}", base_url, entity_str, value)
}
