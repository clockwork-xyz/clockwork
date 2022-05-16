use {
    crate::{cli::CliError, utils::sign_and_submit},
    solana_client_helpers::Client,
    solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey},
    std::sync::Arc,
};

pub fn initialize(client: &Arc<Client>, mint: Pubkey) -> Result<(), CliError> {
    // Initialize the programs
    let admin = client.payer_pubkey();
    let ix_a = cronos_sdk::healthcheck::instruction::initialize(admin);
    let ix_b = cronos_sdk::scheduler::instruction::initialize(admin);
    let ix_c = cronos_sdk::network::instruction::initialize(admin, mint);
    let ix_d = cronos_sdk::pool::instruction::initialize(admin);

    // Fund the network program's yogi
    let authority = cronos_sdk::network::state::Authority::pda().0;
    let yogi = cronos_sdk::scheduler::state::Yogi::pda(authority).0;
    let ix_e = cronos_sdk::scheduler::instruction::yogi_fund(LAMPORTS_PER_SOL / 4, admin, yogi);

    // Submit tx
    sign_and_submit(client, &[ix_a, ix_b, ix_c, ix_d, ix_e], &[client.payer()]);
    Ok(())
}
