use cronos_client::Client;

use crate::errors::CliError;

pub fn get(_client: &Client) -> Result<(), CliError> {
    panic!("Not implemented – moving to health program")
    // let health_addr = cronos_client::account::Health::pda().0;
    // let data = client
    //     .get_account_data(&health_addr)
    //     .map_err(|_err| CliError::AccountNotFound(health_addr.to_string()))?;
    // let health_data = cronos_client::account::Health::try_from(data)
    //     .map_err(|_err| CliError::AccountDataNotParsable(health_addr.to_string()))?;
    // let ts =
    //     cronos_client::clock::get_time(client).map_err(|err| CliError::BadClient(err.to_string()))?;

    // println!("  Block time: {}", ts);
    // println!("   Last ping: {} sec", ts - health_data.last_ping);
    // println!("Recurr drift: {} sec", ts - health_data.target_ping);
    // Ok(())
}
