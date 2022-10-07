use {
    crate::errors::CliError,
    clockwork_client::{pool::objects::Pool, Client},
    solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey},
};

pub fn initialize(client: &Client, mint: Pubkey) -> Result<(), CliError> {
    // Create a worker pool
    let pool_name = "queue";
    let pool = Pool::pubkey(pool_name.into());

    // Initialize the programs
    let admin = client.payer_pubkey();
    let ix_a = clockwork_client::network::instruction::initialize(admin, mint);
    let ix_b = clockwork_client::network::instruction::pool_create(admin, pool_name.into(), 1);

    // Submit tx
    client
        .send_and_confirm(&[ix_a, ix_b, ix_c, ix_d, ix_e], &[client.payer()])
        .unwrap();

    Ok(())
}
