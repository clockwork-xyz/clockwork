use {
    solana_clap_utils::keypair::DefaultSigner,
    solana_client_helpers::{Client, RpcClient},
    solana_sdk::{
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        native_token::LAMPORTS_PER_SOL,
        signature::{read_keypair, Keypair, Signer},
        transaction::Transaction,
    },
    std::{fs::File, sync::Arc},
};

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

pub fn _load_keypair(path: &String) -> Keypair {
    let signer = DefaultSigner::new("keypair".to_string(), &path);
    read_keypair(&mut File::open(signer.path.as_str()).unwrap()).unwrap()
}

pub fn sign_and_submit(client: &Arc<Client>, ixs: &[Instruction], signer: &Keypair) {
    let mut tx = Transaction::new_with_payer(ixs, Some(&signer.pubkey()));
    tx.sign(&[signer], client.latest_blockhash().unwrap());
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
    let entity_path = match entity {
        SolanaExplorerAccountType::Tx => "tx",
    };
    format!("{}/{}/{}", base_url, entity_path, value)
}
