use {
    crate::errors::CliError,
    clockwork_client::{network::objects::Pool, Client},
    solana_sdk::pubkey::Pubkey,
};

pub fn initialize(client: &Client, mint: Pubkey) -> Result<(), CliError> {
    // Initialize the programs
    let admin = client.payer_pubkey();
    let ix_a = clockwork_client::network::instruction::initialize(admin, mint);
    let ix_b = clockwork_client::network::instruction::pool_create(
        admin,
        "queue".into(),
        admin,
        Pool::pubkey("queue".into()),
        1,
    );

    // Submit tx
    client
        .send_and_confirm(&[ix_a, ix_b], &[client.payer()])
        .unwrap();

    Ok(())
}
