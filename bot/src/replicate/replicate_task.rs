use anchor_client::anchor_lang::prelude::borsh;
use cronos_sdk::account::*;
use solana_sdk::pubkey::Pubkey;

use crate::env;

pub fn replicate_task(task: Pubkey, task_data: Task) {
    println!("💽 Replicate task: {} {}", task, task_data.status);

    // Build postgres client
    let mut psql = postgres::Client::connect(env::psql_params().as_str(), postgres::NoTls).unwrap();

    // Write task to postgres
    let query = "INSERT INTO tasks 
        (pubkey, daemon, status, exec_at, ix) 
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (pubkey) DO UPDATE SET
        status = EXCLUDED.status,
        exec_at = EXCLUDED.exec_at,
        ix = EXCLUDED.ix";
    psql.execute(
        query,
        &[
            &task.to_string(),
            &task_data.daemon.to_string(),
            &task_data.status.to_string(),
            &task_data.schedule.exec_at,
            &borsh::try_to_vec_with_schema(&task_data.ix).unwrap(),
        ],
    )
    .unwrap();
}
