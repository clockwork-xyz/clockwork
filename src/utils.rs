use std::{fs::File, sync::Arc};

use solana_clap_utils::keypair::DefaultSigner;
use solana_client_helpers::Client;
use solana_sdk::{
    instruction::Instruction,
    signature::{read_keypair, Keypair},
    transaction::Transaction,
};

use crate::config::CliConfig;

pub fn load_keypair(config: &CliConfig) -> Keypair {
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
    Account,
    Tx,
}

pub fn solana_explorer_url(entity: SolanaExplorerAccountType, value: String) -> String {
    let base_url = "https://explorer.solana.com";
    let entity_str = match entity {
        SolanaExplorerAccountType::Account => "address",
        SolanaExplorerAccountType::Tx => "tx",
    };
    let cluster_params = "cluster=devnet";
    format!("{}/{}/{}?{}", base_url, entity_str, value, cluster_params)
}
