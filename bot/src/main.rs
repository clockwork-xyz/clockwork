use std::sync::{Arc, RwLock};

use cache::{MutableTaskCache, TaskCache};
use dotenv::dotenv;
use solana_client_helpers::ClientResult;
use utils::new_rpc_client;

mod cache;
mod env;
mod exec;
mod replicate;
mod utils;

use {exec::*, replicate::*};

fn main() -> ClientResult<()> {
    // Load env file
    dotenv().ok();

    // Load resources
    let client = Arc::new(new_rpc_client());
    let cache = Arc::new(RwLock::new(TaskCache::new()));

    // Replicate tasks
    replicate_tasks(cache.clone());

    // Execute tasks
    execute_tasks(client, cache);

    Ok(())
}
