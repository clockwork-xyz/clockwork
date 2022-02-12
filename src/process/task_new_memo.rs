use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{error::CliError, signer::sign_and_submit};

pub fn process(
    client: &Arc<Client>,
    memo: String,
    exec_at: Option<i64>,
    stop_at: Option<i64>,
    recurr: Option<i64>,
) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let daemon_addr = cronos_sdk::account::Daemon::find_pda(owner).0;

    let data = client
        .get_account_data(&daemon_addr)
        .map_err(|_err| CliError::AccountNotFound(daemon_addr.to_string()))?;
    let daemon_data = cronos_sdk::account::Daemon::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(daemon_addr.to_string()))?;

    let memo_ix = spl_memo::build_memo(memo.as_bytes(), &[&daemon_addr]);

    let task_pda = cronos_sdk::account::Task::find_pda(daemon_addr, daemon_data.task_count);
    let config_addr = cronos_sdk::account::Config::find_pda().0;

    let exec_at = match exec_at {
        Some(v) => v,
        None => cronos_sdk::blocktime::blocktime(client)
            .map_err(|err| CliError::BadClient(err.to_string()))?,
    };
    let recurr = match recurr {
        Some(v) => v,
        None => 0,
    };
    let stop_at = match stop_at {
        Some(v) => v,
        None => {
            if recurr == 0 {
                exec_at
            } else {
                i64::MAX
            }
        }
    };

    let ix = cronos_sdk::instruction::task_create(
        task_pda,
        config_addr,
        daemon_addr,
        owner,
        memo_ix,
        exec_at,
        stop_at,
        recurr,
    );

    sign_and_submit(client, &[ix]);
    Ok(())
}
