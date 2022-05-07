use {
    crate::{cli::CliError, utils::sign_and_submit},
    solana_client_helpers::Client,
    solana_sdk::pubkey::Pubkey,
    std::sync::Arc,
};

pub fn initialize(client: &Arc<Client>, mint: Pubkey) -> Result<(), CliError> {
    // Common
    let admin = client.payer_pubkey();

    // Initialize the heartbeat program
    let config_pda = cronos_sdk::heartbeat::state::Config::pda();
    let heartbeat_pda = cronos_sdk::heartbeat::state::Heartbeat::pda();
    let ix_a = cronos_sdk::heartbeat::instruction::initialize(admin, config_pda, heartbeat_pda);

    // Initialize the network program
    let config_pda = cronos_sdk::network::state::Config::pda();
    let pool_pda = cronos_sdk::network::state::Pool::pda();
    let registry_pda = cronos_sdk::network::state::Registry::pda();
    let snapshot_pda = cronos_sdk::network::state::Snapshot::pda(0);
    let ix_b = cronos_sdk::network::instruction::initialize(
        admin,
        mint,
        config_pda,
        pool_pda,
        registry_pda,
        snapshot_pda,
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
    sign_and_submit(client, &[ix_a, ix_b, ix_c]);
    Ok(())
}
