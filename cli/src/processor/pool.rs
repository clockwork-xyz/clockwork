use {
    crate::errors::CliError,
    clockwork_client::{pool::state::Pool, Client},
};

pub fn get(client: &Client) -> Result<(), CliError> {
    let pool_pubkey = Pool::pubkey();
    let pool = client
        .get::<Pool>(&pool_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(pool_pubkey.to_string()))?;
    println!("{:#?}", pool);
    Ok(())
}
