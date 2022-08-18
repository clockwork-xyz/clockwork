use {crate::errors::CliError, clockwork_client::Client, solana_sdk::pubkey::Pubkey};

pub fn initialize(client: &Client, mint: Pubkey) -> Result<(), CliError> {
    // Initialize the programs
    let admin = client.payer_pubkey();
    let ix_a = clockwork_client::http::instruction::initialize(admin);
    let ix_b = clockwork_client::crank::instruction::initialize(admin);
    let ix_c = clockwork_client::network::instruction::initialize(admin, mint);
    let ix_d = clockwork_client::pool::instruction::initialize(admin);

    // Submit tx
    client
        .send_and_confirm(&[ix_a, ix_b, ix_c, ix_d], &[client.payer()])
        .unwrap();
    Ok(())
}
