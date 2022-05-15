use {
    crate::{
        cli::CliError,
        utils::{solana_explorer_url, SolanaExplorerAccountType},
    },
    cronos_sdk::pool::state::Pool,
    solana_client_helpers::Client,
    std::sync::Arc,
};

pub fn get(client: &Arc<Client>) -> Result<(), CliError> {
    let address = Pool::pda().0;
    let data = client
        .get_account_data(&address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let data = Pool::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "Explorer: {}",
        solana_explorer_url(SolanaExplorerAccountType::Account, address.to_string())
    );
    println!("{:#?}", data);
    Ok(())
}
