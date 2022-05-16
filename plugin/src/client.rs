use {
    async_trait::async_trait,
    log::info,
    solana_client_helpers::{Client, ClientResult, RpcClient},
    solana_sdk::{
        commitment_config::CommitmentConfig, instruction::Instruction, signature::read_keypair,
        signature::Signature, transaction::Transaction,
    },
    std::fs::File,
};

#[async_trait]
pub trait RPCClient {
    fn new(keypath: String, url: String) -> Client;
    async fn sign_and_submit(&self, ixs: &[Instruction], memo: &str) -> ClientResult<Signature>;
}

#[async_trait]
impl RPCClient for Client {
    fn new(keypath: String, url: String) -> Client {
        let payer = read_keypair(&mut File::open(keypath).unwrap()).unwrap();
        let client = RpcClient::new_with_commitment::<String>(url, CommitmentConfig::confirmed());
        Client { client, payer }
    }

    async fn sign_and_submit(&self, ixs: &[Instruction], memo: &str) -> ClientResult<Signature> {
        info!("{}", memo);
        let payer = self.payer_pubkey();
        let mut tx = Transaction::new_with_payer(ixs, Some(&payer));
        tx.sign(&vec![&self.payer], self.latest_blockhash()?);
        let sig = self.send_and_confirm_transaction(&tx)?;
        info!("✅ {:?}", sig);
        Ok(sig)
    }
}
