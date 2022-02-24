use anchor_lang::prelude::Pubkey;
use cronos_sdk::account::*;

use crate::env;

pub fn replicate_task(pubkey: Pubkey, task: Task) {
    println!("💽 Replicate task: {} {}", pubkey, task.status);

    // Build postgres client
    let mut psql = postgres::Client::connect(env::psql_params().as_str(), postgres::NoTls).unwrap();

    // Write task to postgres
    let query = "INSERT INTO tasks 
        (pubkey, daemon, status, exec_at) 
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (pubkey) DO UPDATE SET
        status = EXCLUDED.status,
        exec_at = EXCLUDED.exec_at";
    psql.execute(
        query,
        &[
            &pubkey.to_string(),
            &task.daemon.to_string(),
            &task.status.to_string(),
            &task.schedule.exec_at,
        ],
    )
    .unwrap();
}
