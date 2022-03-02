use {
    crate::env,
    solana_client_helpers::{Client, ClientResult, RpcClient},
    solana_sdk::{
        commitment_config::CommitmentConfig, instruction::Instruction, signature::read_keypair,
        signature::Signature, transaction::Transaction,
    },
    std::fs::File,
};

pub trait RPCClient {
    fn new() -> Client;
    fn sign_and_submit(&self, ixs: &[Instruction], memo: &str) -> ClientResult<Signature>;
}

impl RPCClient for Client {
    fn new() -> Client {
        let payer = read_keypair(&mut File::open(env::keypath().as_str()).unwrap()).unwrap();
        let client = RpcClient::new_with_commitment::<String>(
            env::rpc_endpoint().as_str().into(),
            CommitmentConfig::processed(),
        );
        Client { client, payer }
    }

    fn sign_and_submit(&self, ixs: &[Instruction], memo: &str) -> ClientResult<Signature> {
        println!("{}", memo);
        let payer = self.payer_pubkey();
        let mut tx = Transaction::new_with_payer(ixs, Some(&payer));
        tx.sign(&vec![&self.payer], self.latest_blockhash()?);
        let sig = self.send_and_confirm_transaction(&tx)?;
        println!("✅ {:?}", sig);
        Ok(sig)
    }
}
