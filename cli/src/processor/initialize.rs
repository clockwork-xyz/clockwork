use {
    crate::cli::CliError,
    cronos_sdk::Client,
    solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey},
};

pub fn initialize(client: &Client, mint: Pubkey) -> Result<(), CliError> {
    // Initialize the programs
    let admin = client.payer_pubkey();
    let ix_a = cronos_sdk::health::instruction::initialize(admin);
    let ix_b = cronos_sdk::scheduler::instruction::initialize(admin);
    let ix_c = cronos_sdk::network::instruction::initialize(admin, mint);
    let ix_d = cronos_sdk::pool::instruction::initialize(admin);

    // Fund the network program's queues
    let authority = cronos_sdk::network::state::Authority::pda().0;
    let manager = cronos_sdk::scheduler::state::Manager::pda(authority).0;
    let queue_0 = cronos_sdk::scheduler::state::Queue::pda(manager, 0).0;
    let queue_1 = cronos_sdk::scheduler::state::Queue::pda(manager, 1).0;
    let ix_e = cronos_sdk::scheduler::instruction::queue_fund(LAMPORTS_PER_SOL, admin, queue_0);
    let ix_f = cronos_sdk::scheduler::instruction::queue_fund(LAMPORTS_PER_SOL, admin, queue_1);

    // Submit tx
    client
        .sign_and_submit(&[ix_a, ix_b, ix_c, ix_d, ix_e, ix_f], &[client.payer()])
        .unwrap();
    Ok(())
}
