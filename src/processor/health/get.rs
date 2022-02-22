use std::sync::Arc;

use solana_client_helpers::Client;

use crate::error::CliError;

pub fn get(client: &Arc<Client>) -> Result<(), CliError> {
    let health_addr = cronos_sdk::account::Health::pda().0;
    let data = client
        .get_account_data(&health_addr)
        .map_err(|_err| CliError::AccountNotFound(health_addr.to_string()))?;
    let health_data = cronos_sdk::account::Health::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(health_addr.to_string()))?;
    let blocktime =
        cronos_sdk::blocktime(client).map_err(|err| CliError::BadClient(err.to_string()))?;
    println!("  Block time: {}", blocktime);
    println!("   Last ping: {} sec", blocktime - health_data.last_ping);
    println!("Recurr drift: {} sec", blocktime - health_data.target_ping);

    // TODO measure change in drift since last query
    // TODO calculate change in drift over 1/sec time frame
    //
    // This metric would measure if Cronos bots are
    // "catching up" up or "falling behind" expected time.

    Ok(())
}
