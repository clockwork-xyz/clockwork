use solana_clap_utils::keypair::DefaultSigner;
use solana_client_helpers::Client;
use solana_sdk::{
    instruction::Instruction,
    signature::{read_keypair, Keypair},
    signers::Signers,
    transaction::Transaction,
};
use std::{fs::File, sync::Arc};

use crate::config::CliConfig;

pub fn load_keypair(config: &CliConfig) -> Keypair {
    let signer = DefaultSigner::new("keypair".to_string(), &config.keypair_path);
    read_keypair(&mut File::open(signer.path.as_str()).unwrap()).unwrap()
}

pub fn sign_and_submit<T: Signers>(client: &Arc<Client>, ixs: &[Instruction], signers: &T) {
    let mut tx = Transaction::new_with_payer(ixs, signers.pubkeys().first());
    tx.sign(signers, client.latest_blockhash().unwrap());
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
    format!("{}/{}/{}", base_url, entity_str, value)
}
