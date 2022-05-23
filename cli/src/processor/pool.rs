use {
    crate::cli::CliError,
    cronos_sdk::{pool::state::Pool, Client},
};

pub fn get(client: &Client) -> Result<(), CliError> {
    let address = Pool::pda().0;
    let pool = client
        .get::<Pool>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", pool);
    Ok(())
}
