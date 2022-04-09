#[cfg(test)]
mod tests {
    use {
        cronos_sdk::scheduler,
        solana_client_helpers::{Client, RpcClient},
        solana_sdk::{
            commitment_config::CommitmentConfig, instruction::Instruction,
            native_token::LAMPORTS_PER_SOL, signature::Keypair, transaction::Transaction,
        },
        std::sync::Arc,
    };

    fn new_client() -> Arc<Client> {
        let url = "http://localhost:8899";
        let client = Arc::new(Client {
            client: RpcClient::new_with_commitment(url, CommitmentConfig::processed()),
            payer: Keypair::new(),
        });
        client
            .airdrop(&client.payer_pubkey(), LAMPORTS_PER_SOL)
            .unwrap();
        client
    }

    fn sign_and_submit(client: &Arc<Client>, ixs: &[Instruction]) {
        let mut tx = Transaction::new_with_payer(ixs, Some(&client.payer_pubkey()));
        tx.sign(&vec![&client.payer], client.latest_blockhash().unwrap());
        let sig = client.send_and_confirm_transaction(&tx).unwrap();
        println!("Signature: {}", sig);
    }

    #[test]
    #[ignore]
    fn initialize() {
        let client = new_client();
        let authority_pda = scheduler::state::Authority::pda();
        let config_pda = scheduler::state::Config::pda();
        let daemon_pda = scheduler::state::Daemon::pda(authority_pda.0);
        let fee_pda = scheduler::state::Fee::pda(daemon_pda.0);
        let ix = scheduler::instruction::admin_initialize(
            client.payer_pubkey(),
            authority_pda,
            config_pda,
            daemon_pda,
            fee_pda,
        );
        sign_and_submit(&client, &[ix]);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
