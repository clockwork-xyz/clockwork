use std::sync::{Arc, Mutex, RwLock};

use bucket::Bucket;
use cache::TaskCache;
use client::RPCClient;
use dotenv::dotenv;
use solana_client_helpers::{Client, ClientResult};

mod bucket;
mod cache;
mod client;
mod env;
mod exec;
mod replicate;

use {exec::*, replicate::*};

fn main() -> ClientResult<()> {
    // Load env file
    dotenv().ok();

    // Load resources
    let client = Arc::new(Client::new());
    let cache = Arc::new(RwLock::new(TaskCache::new()));
    let bucket = Arc::new(Mutex::new(Bucket::default()));

    // Replicate tasks
    replicate_tasks(cache.clone());

    // Execute tasks
    execute_tasks(client, cache, bucket);

    Ok(())
}
