use clockwork_client::pool::state::Pool;

use {
    crate::errors::CliError,
    clockwork_client::Client,
    solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey},
};

pub fn initialize(client: &Client, mint: Pubkey) -> Result<(), CliError> {
    // TODO Create a worker pool
    let pool_name = "crank";
    let pool = Pool::pubkey(pool_name.into());

    // Initialize the programs
    let admin = client.payer_pubkey();
    let ix_a = clockwork_client::crank::instruction::initialize(admin, pool);
    let ix_b = clockwork_client::http::instruction::initialize(admin);
    let ix_c = clockwork_client::network::instruction::initialize(admin, mint);
    let ix_d = clockwork_client::network::instruction::pool_create(admin, pool_name.into(), 1);

    // Submit tx
    client
        .send_and_confirm(&[ix_a, ix_b, ix_c, ix_d], &[client.payer()])
        .unwrap();

    // Airdrop some lamports to the network's snapshot queue
    let network_authority_pubkey = clockwork_client::network::state::Authority::pubkey();
    let snapshot_queue_pubkey =
        clockwork_client::crank::state::Queue::pubkey(network_authority_pubkey, "snapshot".into());
    client
        .airdrop(&snapshot_queue_pubkey, LAMPORTS_PER_SOL)
        .unwrap();

    Ok(())
}
