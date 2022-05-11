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
    let config = cronos_sdk::heartbeat::state::Config::pda().0;
    let heartbeat = cronos_sdk::heartbeat::state::Heartbeat::pda().0;
    let ix_a = cronos_sdk::heartbeat::instruction::initialize(admin, config, heartbeat);

    // Initialize scheduler program
    let authority = cronos_sdk::scheduler::state::Authority::pda().0;
    let config = cronos_sdk::scheduler::state::Config::pda().0;
    let queue = cronos_sdk::scheduler::state::Queue::pda(authority).0;
    let fee = cronos_sdk::scheduler::state::Fee::pda(queue).0;
    let ix_b = cronos_sdk::scheduler::instruction::initialize(
        admin, authority, config, fee,
        admin, // TODO 'admin' is just a placeholder. Pass in correct pool id after building out the pool program.
        queue,
    );

    // Initialize network program
    let authority = cronos_sdk::network::state::Authority::pda().0;
    let config = cronos_sdk::network::state::Config::pda().0;
    let pool = cronos_sdk::network::state::Pool::pda().0;
    let registry = cronos_sdk::network::state::Registry::pda().0;
    let snapshot = cronos_sdk::network::state::Snapshot::pda(0).0;

    let queue = cronos_sdk::scheduler::state::Queue::pda(authority).0;
    let fee = cronos_sdk::scheduler::state::Fee::pda(queue).0;
    let task = cronos_sdk::scheduler::state::Task::pda(queue, 0).0;

    let ix_c = cronos_sdk::network::instruction::initialize(
        admin, authority, config, fee, mint, pool, queue, registry, snapshot, task,
    );

    // Submit tx
    sign_and_submit(client, &[ix_a, ix_b, ix_c]);
    Ok(())
}
