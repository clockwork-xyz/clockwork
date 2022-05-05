use {
    crate::cli::CliError,
    solana_client_helpers::Client,
    solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction},
    std::sync::Arc,
};

pub fn initialize(client: &Arc<Client>) -> Result<(), CliError> {
    // Common
    let admin = client.payer_pubkey();
    let mint = Keypair::new();

    // Initialize the heartbeat program
    let config_pda = cronos_sdk::heartbeat::state::Config::pda();
    let heartbeat_pda = cronos_sdk::heartbeat::state::Heartbeat::pda();
    let ix_a = cronos_sdk::heartbeat::instruction::initialize(admin, config_pda, heartbeat_pda);

    // Initialize the network program
    let config_pda = cronos_sdk::network::state::Config::pda();
    let pool_pda = cronos_sdk::network::state::Pool::pda();
    let registry_pda = cronos_sdk::network::state::Registry::pda();
    let registry_page_pda = cronos_sdk::network::state::RegistryPage::pda(0);
    let snapshot_pda = cronos_sdk::network::state::Snapshot::pda(0);
    let snapshot_page_pda = cronos_sdk::network::state::SnapshotPage::pda(snapshot_pda.0, 0);
    let ix_b = cronos_sdk::network::instruction::initialize(
        admin,
        mint.pubkey(),
        config_pda,
        pool_pda,
        registry_pda,
        registry_page_pda,
        snapshot_pda,
        snapshot_page_pda,
    );

    // Initialize scheduler program
    let authority_pda = cronos_sdk::scheduler::state::Authority::pda();
    let config_pda = cronos_sdk::scheduler::state::Config::pda();
    let daemon_pda = cronos_sdk::scheduler::state::Daemon::pda(authority_pda.0);
    let fee_pda = cronos_sdk::scheduler::state::Fee::pda(daemon_pda.0);
    let ix_c = cronos_sdk::scheduler::instruction::initialize(
        admin,
        authority_pda,
        config_pda,
        daemon_pda,
        fee_pda,
        registry_pda.0,
    );

    // Submit tx
    let mut tx = Transaction::new_with_payer(&[ix_a, ix_b, ix_c], Some(&client.payer_pubkey()));
    tx.sign(
        &vec![&client.payer, &mint],
        client.latest_blockhash().unwrap(),
    );
    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    println!("Tx: {}", sig.to_string());

    Ok(())
}
