use {
    crate::errors::CliError,
    cronos_client::Client,
    solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey},
};

pub fn initialize(client: &Client, mint: Pubkey) -> Result<(), CliError> {
    // Initialize the programs
    let admin = client.payer_pubkey();
    let ix_a = cronos_client::health::instruction::initialize(admin);
    let ix_b = cronos_client::scheduler::instruction::initialize(admin);
    let ix_c = cronos_client::network::instruction::initialize(admin, mint);
    let ix_d = cronos_client::pool::instruction::initialize(admin);

    // Fund the network program's queues
    let authority = cronos_client::network::state::Authority::pda().0;
    let manager = cronos_client::scheduler::state::Manager::pubkey(authority);
    let ix_e =
        cronos_client::scheduler::instruction::manager_fund(LAMPORTS_PER_SOL, manager, admin);

    // Submit tx
    client
        .send_and_confirm(&[ix_a, ix_b, ix_c, ix_d, ix_e], &[client.payer()])
        .unwrap();
    Ok(())
}
