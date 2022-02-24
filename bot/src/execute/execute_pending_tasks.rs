use {
    crate::{env, execute_task},
    anchor_lang::prelude::Pubkey,
    cronos_sdk::account::*,
    std::{str::FromStr, thread},
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
        thread::spawn(move || execute_task(task, daemon));
    }
}
