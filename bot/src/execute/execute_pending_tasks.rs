use solana_sdk::pubkey::Pubkey;

use {
    crate::{env, execute_task},
    anchor_client::anchor_lang::prelude::borsh,
    cronos_sdk::account::*,
    std::str::FromStr,
};

pub fn execute_pending_tasks(blocktime: i64) {
    let mut psql = postgres::Client::connect(env::psql_params().as_str(), postgres::NoTls).unwrap();
    let query = "SELECT * FROM tasks WHERE status = $1 AND exec_at <= $2";
    for row in psql
        .query(query, &[&TaskStatus::Queued.to_string(), &blocktime])
        .unwrap()
    {
        let task = Pubkey::from_str(row.get(0)).unwrap();
        let daemon = Pubkey::from_str(row.get(1)).unwrap();
        let ix_bytes: Vec<u8> = row.get(4);
        let ix = borsh::try_from_slice_with_schema(ix_bytes.as_slice()).unwrap();
        execute_task(task, daemon, ix);
    }
}
