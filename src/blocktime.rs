use std::sync::Arc;

use anchor_client::ClientError;
use solana_client_helpers::Client;

pub fn blocktime(client: &Arc<Client>) -> Result<i64, ClientError> {
    let slot = client.get_slot()?;
    let blocktime = client.get_block_time(slot)?;
    Ok(blocktime)
}
